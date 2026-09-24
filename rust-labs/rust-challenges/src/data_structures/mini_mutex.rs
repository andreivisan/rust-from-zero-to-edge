use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
};

// The mutex must handle three things:
// - Exclusion: only the successful lock holder accesses the value.
// - Ordering: the next holder sees the previous holder's changes.
// - Waiting: contenders retry or sleep. Our version retries.
struct MiniMutex<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

struct MiniMutexGuard<'a, T> {
    mutex: &'a MiniMutex<T>,
    _marker: PhantomData<*mut ()>,
}

impl<T> MiniMutex<T> {
    pub fn new(input: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(input),
        }
    }

    pub fn lock(&self) -> MiniMutexGuard<'_, T> {
        loop {
            match self.locked.compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_value) => {
                    return MiniMutexGuard {
                        mutex: self,
                        _marker: PhantomData,
                    };
                }
                Err(_err) => {
                    spin_loop();
                    continue;
                }
            }
        }
    }
}

impl<'a, T> Drop for MiniMutexGuard<'a, T> {
    fn drop(&mut self) {
        (*self.mutex)
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<'a, T> Deref for MiniMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let raw_value = UnsafeCell::get(&self.mutex.value);
        unsafe { &(*raw_value) }
    }
}

impl<'a, T> DerefMut for MiniMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let raw_value = UnsafeCell::get(&self.mutex.value);
        unsafe { &mut (*raw_value) }
    }
}

unsafe impl<T: Send> Sync for MiniMutex<T> {}

#[cfg(test)]
mod tests {
    use super::MiniMutex;
    use std::{
        cell::Cell,
        panic::{AssertUnwindSafe, catch_unwind},
        sync::{Barrier, atomic::Ordering},
        thread,
    };

    #[test]
    fn guard_reads_and_mutates_a_non_copy_value() {
        let mutex = MiniMutex::new(String::from("Hello"));

        {
            let mut guard = mutex.lock();
            guard.push_str(", Rust!");
            let message: &str = &guard;
            assert_eq!(message, "Hello, Rust!");
        }

        // Check unlocking before reacquiring so a broken Drop fails, not spins forever.
        assert!(!mutex.locked.load(Ordering::Relaxed));
        let guard = mutex.lock();
        assert_eq!(guard.as_str(), "Hello, Rust!");
    }

    #[test]
    fn lock_stays_held_until_its_guard_is_dropped() {
        let mutex = MiniMutex::new(42);
        let guard = mutex.lock();

        assert!(mutex.locked.load(Ordering::Relaxed));

        let moved_guard = guard;
        assert!(mutex.locked.load(Ordering::Relaxed));

        drop(moved_guard);
        assert!(!mutex.locked.load(Ordering::Relaxed));
    }

    #[test]
    fn the_same_thread_can_acquire_the_mutex_repeatedly() {
        let mutex = MiniMutex::new(0);

        for _ in 0..10 {
            assert!(!mutex.locked.load(Ordering::Relaxed));
            let mut guard = mutex.lock();
            *guard += 1;
            // Each iteration ends one guard's lifetime, not the thread's lifetime.
        }

        assert!(!mutex.locked.load(Ordering::Relaxed));
        assert_eq!(*mutex.lock(), 10);
    }

    #[test]
    fn threads_take_turns_updating_a_send_but_not_sync_value() {
        const THREADS: usize = 4;
        const UPDATES_PER_THREAD: usize = 2_000;

        // Cell is Send but not Sync. Our mutex must make sharing it safe.
        let mutex = MiniMutex::new(Cell::new(0usize));
        let start = Barrier::new(THREADS);

        thread::scope(|scope| {
            for _ in 0..THREADS {
                scope.spawn(|| {
                    start.wait();

                    for update in 0..UPDATES_PER_THREAD {
                        let guard = mutex.lock();
                        let previous = guard.get();

                        // Give contenders a chance to run while this guard holds the lock.
                        if update % 100 == 0 {
                            thread::yield_now();
                        }

                        guard.set(previous + 1);
                    }
                });
            }
        });

        assert!(!mutex.locked.load(Ordering::Relaxed));
        assert_eq!(mutex.lock().get(), THREADS * UPDATES_PER_THREAD);
    }

    #[test]
    fn dropping_a_guard_preserves_the_value_until_the_mutex_is_dropped() {
        struct DropTracker<'a>(&'a Cell<usize>);

        impl Drop for DropTracker<'_> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let drops = Cell::new(0);

        {
            let mutex = MiniMutex::new(DropTracker(&drops));
            let guard = mutex.lock();
            drop(guard);

            assert_eq!(drops.get(), 0, "unlocking must not destroy the value");
        }

        assert_eq!(drops.get(), 1, "destroying the mutex drops its value once");
    }

    #[test]
    fn guard_unlocks_during_panic_unwinding() {
        let mutex = MiniMutex::new(0);

        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut guard = mutex.lock();
            *guard = 7;
            panic!("intentional panic to test guard cleanup");
        }));

        assert!(result.is_err());
        assert!(!mutex.locked.load(Ordering::Relaxed));
        // This educational mutex unlocks but does not implement poisoning.
        assert_eq!(*mutex.lock(), 7);
    }
}
