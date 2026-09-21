use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error},
    ops::{Deref, DerefMut},
    ptr::drop_in_place,
};

struct MiniBox<T> {
    ptr: *mut T,
}

impl<T> MiniBox<T> {
    pub fn new(value: T) -> Self {
        let mem_layout = Layout::new::<T>();
        assert!(
            mem_layout.size() > 0,
            "Layout size must be greater than zero!"
        );
        unsafe {
            let ptr = alloc(mem_layout);
            if ptr.is_null() {
                handle_alloc_error(mem_layout);
            }
            (ptr as *mut T).write(value);
            Self { ptr: ptr as *mut T }
        }
    }
}

impl<T> Drop for MiniBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.ptr);
            dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }
}

impl<T> Deref for MiniBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for MiniBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::MiniBox;
    use std::{cell::Cell, mem::align_of, ptr};

    #[test]
    fn reads_the_owned_value() {
        let boxed = MiniBox::new(42);

        assert_eq!(*boxed, 42);
    }

    #[test]
    fn mutates_the_owned_value() {
        let mut boxed = MiniBox::new(42);

        *boxed = 99;

        assert_eq!(*boxed, 99);
    }

    #[test]
    fn owns_a_string_and_supports_deref_coercion() {
        // String is not Copy and has its own heap allocation to clean up.
        let mut boxed = MiniBox::new(String::from("Hello"));

        boxed.push_str(", Rust!");
        let message: &str = &boxed;

        assert_eq!(message, "Hello, Rust!");
    }

    #[test]
    fn moving_preserves_the_allocation_and_drops_the_value_once() {
        struct DropTracker<'a> {
            drops: &'a Cell<usize>,
        }

        impl Drop for DropTracker<'_> {
            fn drop(&mut self) {
                self.drops.set(self.drops.get() + 1);
            }
        }

        let drops = Cell::new(0);

        {
            let boxed = MiniBox::new(DropTracker { drops: &drops });
            let original_address = ptr::from_ref(&*boxed);

            let moved = boxed;

            assert!(ptr::eq(original_address, ptr::from_ref(&*moved)));
            assert_eq!(drops.get(), 0, "moving must not destroy the value");
            // The new owner goes out of scope here and performs cleanup.
        }

        assert_eq!(drops.get(), 1, "the contained value must be dropped once");
    }

    #[test]
    fn respects_the_values_alignment() {
        #[repr(align(64))]
        struct Aligned(u8);

        let boxed = MiniBox::new(Aligned(7));
        let address = ptr::from_ref(&*boxed) as usize;

        assert_eq!(address % align_of::<Aligned>(), 0);
        assert_eq!(boxed.0, 7);
    }

    #[test]
    #[should_panic(expected = "Layout size must be greater than zero!")]
    fn rejects_zero_sized_types() {
        // Unit occupies zero bytes, which this exercise deliberately rejects.
        let _boxed = MiniBox::new(());
    }
}
