use core::{ptr::NonNull, task::Waker};

pub struct Task {}

impl Task {
    pub unsafe fn from_waker(waker: Waker) -> NonNull<Self> {
        NonNull::new_unchecked(waker.as_raw().data() as *mut Self)
    }

    pub fn enqueue(&self) {}
}
