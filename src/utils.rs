use skyline::nn;
use std::ops::{Deref, DerefMut};

pub struct NnMutexGuard<'a, T> {
    mutex: *mut nn::os::MutexType,
    locked_data: &'a mut T
}

impl<'a, T> NnMutexGuard<'a, T> {
    pub fn new(mutex: *mut nn::os::MutexType, data: &'a mut T) -> Self {
        unsafe {
            nn::os::LockMutex(mutex);
        }
        Self {
            mutex,
            locked_data: data
        }
    }
}

impl<'a, T> Drop for NnMutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            nn::os::UnlockMutex(self.mutex);
        }
    }
}

impl<'a, T> Deref for NnMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.locked_data
    }
}

impl<'a, T> DerefMut for NnMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.locked_data
    }
}