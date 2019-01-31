#![no_main]
#![no_std]
#![feature(asm)]

extern crate panic_semihosting;

use core::ptr::read_volatile;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::register::{msp, psp};
use cortex_m::Peripherals;
use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprintln,
};
use cortex_m_switch::{exception, ExceptionReturn};

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(16_000);
    syst.enable_counter();
    syst.enable_interrupt();

    unsafe { asm!("svc #0") };

    loop {}
}

#[exception]
fn SysTick(exc: ExceptionReturn) -> ExceptionReturn {
    static mut COUNTER: usize = 0;

    *COUNTER += 1;
    hprintln!("{:02} ({:?})", *COUNTER, exc).unwrap();
    if *COUNTER == 10 {
        debug::exit(EXIT_SUCCESS);
    }
    ExceptionReturn::ThreadMsp
}

#[exception]
fn PendSV(exc: ExceptionReturn) -> ExceptionReturn {
    exc
}

#[exception]
fn SVCall(exc: ExceptionReturn) -> ExceptionReturn {
    hprintln!("System call!").unwrap();
    let stack_ptr = match exc {
        ExceptionReturn::ThreadMsp => msp::read(),
        ExceptionReturn::ThreadPsp => psp::read(),
        ExceptionReturn::HandlerMsp => msp::read(),
    };
    hprintln!("Stack ptr: {:x}", stack_ptr).unwrap();
    let frame_ptr = stack_ptr as *const ExceptionFrame;
    let frame = unsafe { read_volatile(frame_ptr) };
    hprintln!("Frame: {:?}", frame).unwrap();
    let inst = unsafe { read_volatile(frame.pc as *const u32) };
    hprintln!("PC Instruction: {:x}", inst);

    exc
}
