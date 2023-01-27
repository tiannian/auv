use core::{
    ptr::null,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::SignalEmiter;

unsafe fn auv_drop(_: *const ()) {}

unsafe fn auv_wake(emiter: *const ()) {
    let emiter = emiter as *const SignalEmiter;
    let emiter = &*emiter;
    emiter.emit(1, 1, &[]);
}

unsafe fn auv_clone(_: *const ()) -> RawWaker {
    RawWaker::new(null(), &RWVT)
}

static RWVT: RawWakerVTable = RawWakerVTable::new(auv_clone, auv_wake, auv_wake, auv_drop);

/// Build waker
pub fn build_waker(emiter: &SignalEmiter) -> Waker {
    let r = RawWaker::new(emiter as *const SignalEmiter as *const (), &RWVT);
    unsafe { Waker::from_raw(r) }
}
