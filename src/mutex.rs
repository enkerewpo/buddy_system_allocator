use core::cell::UnsafeCell;

pub struct Mutex<T> {
  inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
  pub const fn new(data: T) -> Mutex<T> {
    Mutex {
      inner: UnsafeCell::new(data),
    }
  }

  pub fn lock(&self) -> &mut T {
    unsafe { &mut *self.inner.get() }
  }
}

unsafe impl<T> Sync for Mutex<T> {
  // todo
}
