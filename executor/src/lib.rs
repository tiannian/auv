#![no_std]

mod waker;
pub use waker::*;

pub mod signal;
#[doc(inline)]
pub use signal::SignalEmiter;

pub mod event;
#[doc(inline)]
pub use event::EventHandler;
