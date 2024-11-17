use std::cell::UnsafeCell;
use std::arch::asm;

pub struct AtomicUsize{
 inner:UnsafeCell<usize>,
}

unsafe impl Send for AtomicUsize {}

unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {

}
  



