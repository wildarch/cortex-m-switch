#![no_std]
extern crate cortex_m_switch_macros as macros;

#[macro_use]
pub mod svc;

pub use macros::exception;

use core::mem;
use core::ptr::read_volatile;
use cortex_m::register::psp;
use cortex_m_rt::ExceptionFrame;

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

/// Registers backed up by the exception handlers for the PendSV, SVCall and SysTick exceptions.
#[derive(Default)]
pub struct SoftwareStackFrame {
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
}

// NOTE Only supports the thumb version of SVC.
pub fn svc_num() -> Option<u8> {
    let stack_ptr = psp::read();
    let stack_ptr = stack_ptr + mem::size_of::<SoftwareStackFrame>() as u32;
    let stack_ptr = stack_ptr as *const ExceptionFrame;
    let frame = unsafe { read_volatile(stack_ptr) };
    let svc_op: u16 = unsafe { read_volatile((frame.pc - 2) as *const u16) };
    let svc_opcode = svc_op >> 8;
    if svc_opcode == 0xDF {
        // The instruction we found is actually an SVC one.

        // Take the lower 8 bits of the instruction containing the immediate.
        let svc_num = svc_op as u8;
        Some(svc_num)
    } else {
        None
    }
}
