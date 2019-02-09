#![no_std]
extern crate cortex_m_switch_macros as macros;

#[macro_use]
pub mod svc;
#[macro_use]
pub mod task;

pub use macros::exception;
pub use task::Task;

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

#[repr(transparent)]
pub struct ExceptionContext(ExceptionReturn);

impl ExceptionContext {
    pub fn exc_return(&self) -> ExceptionReturn {
        self.0
    }

    // NOTE Only supports the thumb version of SVC.
    // NOTE Only works if the interrupt was triggered while using the Process Stack Pointer.
    pub fn svc_num(&self) -> Option<u8> {
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

    pub fn exception_frame(&self) -> Option<&ExceptionFrame> {
        use ExceptionReturn::*;
        match self.exc_return() {
            // We are pushing data on msp ourselves, so we have no way to find the frame again.
            HandlerMsp | ThreadMsp => None,
            #[cfg(has_fpu)]
            HandlerMspFpu | ThreadMspFpu => None,

            ThreadPsp => Some(self.ex_frame()),
            #[cfg(has_fpu)]
            ThreadPspFpu => Some(self.ex_frame()),
        }
    }

    pub fn software_frame(&self) -> Option<&SoftwareStackFrame> {
        use ExceptionReturn::*;
        match self.exc_return() {
            // We are pushing data on msp ourselves, so we have no way to find the frame again.
            HandlerMsp | ThreadMsp => None,
            #[cfg(has_fpu)]
            HandlerMspFpu | ThreadMspFpu => None,

            ThreadPsp => Some(self.sw_frame()),
            #[cfg(has_fpu)]
            ThreadPspFpu => Some(self.sw_frame()),
        }
    }

    fn ex_frame(&self) -> &ExceptionFrame {
        let stack_ptr = psp::read();
        let ex_frame_ptr = stack_ptr + mem::size_of::<SoftwareStackFrame>() as u32;
        let ex_frame_ptr = ex_frame_ptr as *const ExceptionFrame;
        unsafe { &*ex_frame_ptr }
    }

    pub fn sw_frame(&self) -> &SoftwareStackFrame {
        let stack_ptr = psp::read();
        let sw_frame_ptr = stack_ptr as *const SoftwareStackFrame;
        unsafe { &*sw_frame_ptr }
    }
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
