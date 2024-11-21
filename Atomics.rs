use std::cell::UnsafeCell;
use std::arch::asm;

pub struct AtomicUsize{
 inner:UnsafeCell<usize>,
}

unsafe impl Send for AtomicUsize {}

unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {

}

pub const fn new(v: usize) -> Self {
    Self {
        inner: UnsafeCell::new(v),
    }
}

pub fn load(&self) -> usize {
    unsafe { *self.inner.get() }
}

pub fn store(&self, v: usize) {
    unsafe {
        asm!(
            "lock; xchg [{address}], {v}",
            address = in(reg) self.inner.get(),
            v = in(reg) v
        );
    }
}

pub fn fetch_add(&self, mut v: usize) -> usize {
    unsafe {
        asm!(
            "lock; xadd [{address}], {v}",
            address = in(reg) self.inner.get(),
            v = inout(reg) v,
        );
    }

    v
}
  



