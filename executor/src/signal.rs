//! Signal is action from core to runtime

/// A static memory segment for Signal Arguments
#[repr(packed)]
pub struct SignalControlBlock {
    pub signal: u32,
    pub sub_signal: u32,
    pub args_begin: *const u8,
    pub args_length: usize,
}

/// Emiter for signal
pub struct SignalEmiter {
    pscb: *mut SignalControlBlock,
}

impl SignalEmiter {
    /// Init signal emiter
    ///
    /// psas is point of SignalArgumentSegment.
    /// This point must be a static area on memory
    pub fn init(pscb: *mut SignalControlBlock) -> Self {
        Self { pscb }
    }

    /// Emit a signal
    pub fn emit(&self, signal: u32, sub_signal: u32, args: &[u8]) {
        extern "C" {
            fn __auv_signal();
        }

        unsafe {
            (*self.pscb).signal = signal;
            (*self.pscb).sub_signal = sub_signal;
            (*self.pscb).args_begin = args.as_ptr();
            (*self.pscb).args_length = args.len();
            __auv_signal();
        }
    }
}
