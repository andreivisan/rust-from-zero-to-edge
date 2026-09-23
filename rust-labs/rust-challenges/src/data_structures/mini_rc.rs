use std::{cell::Cell, ops::Deref};

struct RcInner<T> {
    strong: Cell<usize>,
    value: T,
}

struct MiniRc<T> {
    ptr: *mut RcInner<T>,
}

impl<T> MiniRc<T> {
    pub fn new(value: T) -> Self {
        let rc_inner = RcInner {
            strong: Cell::new(1),
            value,
        };
        let inner_struct = Box::new(rc_inner);
        let ptr = Box::into_raw(inner_struct);
        MiniRc { ptr }
    }

    pub fn strong_count(&self) -> usize {
        unsafe { Cell::get(&(*self.ptr).strong) }
    }
}

impl<T> Clone for MiniRc<T> {
    fn clone(&self) -> Self {
        let mut counter = self.strong_count();
        counter = counter.checked_add(1).expect("Overflow");
        unsafe {
            (*self.ptr).strong.set(counter);
            MiniRc { ptr: self.ptr }
        }
    }
}

impl<T> Deref for MiniRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).value }
    }
}

impl<T> Drop for MiniRc<T> {
    fn drop(&mut self) {
        let mut counter = self.strong_count();
        counter = counter.checked_sub(1).expect("Counter can't be negative");
        unsafe {
            (*self.ptr).strong.set(counter);
            if counter > 0 {
                return;
            }
            let bx = Box::from_raw(self.ptr);
            drop(bx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MiniRc;
    use std::{cell::Cell, ptr};

    #[test]
    fn starts_with_one_owner_and_borrows_the_value() {
        let owner = MiniRc::new(String::from("shared Rust"));
        let message: &str = &owner;

        assert_eq!(message, "shared Rust");
        assert_eq!(owner.strong_count(), 1);
    }

    #[test]
    fn clone_shares_the_allocation_without_requiring_clone_on_the_value() {
        // This value deliberately does not implement Clone.
        struct NotClone {
            number: i32,
        }

        let first = MiniRc::new(NotClone { number: 42 });
        let second = first.clone();

        assert!(ptr::eq(&*first, &*second));
        assert_eq!(second.number, 42);
        assert_eq!(first.strong_count(), 2);
        assert_eq!(second.strong_count(), 2);
    }

    #[test]
    fn every_owner_observes_the_same_changing_count() {
        let first = MiniRc::new(42);
        let second = first.clone();
        let third = second.clone();

        assert_eq!(first.strong_count(), 3);
        assert_eq!(second.strong_count(), 3);
        assert_eq!(third.strong_count(), 3);

        drop(second);
        assert_eq!(first.strong_count(), 2);
        assert_eq!(third.strong_count(), 2);

        drop(first);
        assert_eq!(third.strong_count(), 1);
        assert_eq!(*third, 42);
    }

    #[test]
    fn moving_and_borrowing_do_not_add_owners() {
        let original = MiniRc::new(42);
        let original_address = ptr::from_ref(&*original);
        let moved = original;

        let borrowed_owner = &moved;
        let borrowed_value: &i32 = &moved;

        assert_eq!(borrowed_owner.strong_count(), 1);
        assert!(ptr::eq(original_address, borrowed_value));
        assert_eq!(*borrowed_value, 42);
    }

    #[test]
    fn only_the_last_owner_drops_the_value() {
        struct DropTracker<'a> {
            drops: &'a Cell<usize>,
            message: String,
        }

        impl Drop for DropTracker<'_> {
            fn drop(&mut self) {
                self.drops.set(self.drops.get() + 1);
            }
        }

        let drops = Cell::new(0);

        {
            let survivor = {
                let original = MiniRc::new(DropTracker {
                    drops: &drops,
                    message: String::from("still alive"),
                });
                original.clone()
                // The original owner disappears, but its clone escapes.
            };

            assert_eq!(drops.get(), 0);
            assert_eq!(survivor.strong_count(), 1);
            assert_eq!(survivor.message.as_str(), "still alive");

            let another = survivor.clone();
            drop(survivor);
            assert_eq!(drops.get(), 0);
            assert_eq!(another.strong_count(), 1);
            // The last owner disappears at the end of this scope.
        }

        assert_eq!(drops.get(), 1);
    }

    #[test]
    fn supports_zero_sized_values() {
        // RcInner still occupies space because it includes the owner count.
        let first = MiniRc::new(());
        let second = first.clone();

        assert_eq!(first.strong_count(), 2);
        drop(first);

        assert_eq!(second.strong_count(), 1);
        assert_eq!(*second, ());
    }

    #[test]
    fn shares_interior_mutation_when_the_value_is_a_cell() {
        let first = MiniRc::new(Cell::new(10));
        let second = first.clone();

        // Cell permits mutation through a shared reference; MiniRc needs no DerefMut.
        second.set(20);

        assert_eq!(first.get(), 20);
        assert_eq!(second.get(), 20);
        assert_eq!(first.strong_count(), 2);
    }
}
