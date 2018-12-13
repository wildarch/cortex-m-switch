#![no_std]
extern crate cortex_m_switch_macros as macros;

pub use macros::exception;

/// Exception return (and entry) codes.
/// See also
/// [Exception return
/// behaviour](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/Babefdjc.html).
#[repr(u32)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ExceptionReturn {
    HandlerMsp = 0xFFFFFFF1,
    ThreadMsp = 0xFFFFFFF9,
    ThreadPsp = 0xFFFFFFFD,

    // TODO implement this feature flag
    #[cfg(has_fpu)]
    HandlerMspFpu = 0xFFFFFFE1,
    #[cfg(has_fpu)]
    ThreadMspFpu = 0xFFFFFFE9,
    #[cfg(has_fpu)]
    ThreadPspFpu = 0xFFFFFFED,
}
