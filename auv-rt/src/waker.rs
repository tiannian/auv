use core::{
    ptr::NonNull,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::Task;

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

unsafe fn wake(p: *const ()) {
    (*(p as *mut Task)).enqueue()
}

unsafe fn drop(_: *const ()) {
    // nop
}

pub fn from_task(p: NonNull<Task>) -> Waker {
    unsafe {
        Waker::from_raw(RawWaker::new(p.as_ptr() as _, &VTABLE))
    }
}

