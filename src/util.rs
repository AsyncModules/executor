use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct SyncSendUnsafeCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Sync> Sync for SyncSendUnsafeCell<T> {}
unsafe impl<T: Send> Send for SyncSendUnsafeCell<T> {}

impl<T> SyncSendUnsafeCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub unsafe fn set(&self, value: T) {
        *self.value.get() = value;
    }

    pub unsafe fn get(&self) -> T
    where
        T: Copy,
    {
        *self.value.get()
    }
}
