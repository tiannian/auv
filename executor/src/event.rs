//! Event is action from runtime to core

use core::{ptr::null, slice::from_raw_parts};

/// ECB (Event Control Block)
///
/// This data structure store on memory
#[repr(packed)]
pub struct EventControlBlock {
    pub signal: u16,
    pub sub_signal: u16,
    pub evt_p: *const (),
    pub evt_l: usize,
}

impl EventControlBlock {
    /// Init ECB
    pub const fn init() -> Self {
        Self {
            signal: 0,
            sub_signal: 0,
            evt_p: null(),
            evt_l: 0,
        }
    }
}

/// Event handler
pub type EventHandler = fn(sub_signal: u16);

static mut ECB: EventControlBlock = EventControlBlock::init();

/// Set event vector table
pub fn set_evt(handlers: &[EventHandler]) {
    unsafe {
        ECB.evt_p = handlers.as_ptr() as *const ();
        ECB.evt_l = handlers.len();
    }
}

/// Call by runtime or compact layer
pub extern "C" fn __auv_event() {
    let (signal, sub_signal, evt) = unsafe {
        let signal = ECB.signal as usize;
        let sub_signal = ECB.sub_signal;

        let evt_p = ECB.evt_p as *const EventHandler;
        let evt_l = ECB.evt_l;

        let evt = from_raw_parts(evt_p, evt_l);

        (signal, sub_signal, evt)
    };

    if let Some(h) = evt.get(signal) {
        h(sub_signal)
    }
}
