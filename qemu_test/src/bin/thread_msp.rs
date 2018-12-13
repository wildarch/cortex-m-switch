#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(asm)]
#![feature(naked_functions)]

extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprint,
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

    loop {}
}

#[exception]
fn SysTick(exc: ExceptionReturn) -> ExceptionReturn {
    static mut COUNTER: usize = 0;

    *COUNTER += 1;
    hprint!("{:02} ({:?})\n", *COUNTER, exc).unwrap();
    if *COUNTER == 10 {
        debug::exit(EXIT_SUCCESS);
    }
    ExceptionReturn::ThreadMsp
}
