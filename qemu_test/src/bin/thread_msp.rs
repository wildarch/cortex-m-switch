#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprintln,
};
use cortex_m_switch::{exception, ExceptionContext, ExceptionReturn};

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
fn SysTick(ctx: ExceptionContext) -> ExceptionReturn {
    static mut COUNTER: usize = 0;

    *COUNTER += 1;
    hprintln!("{:02} ({:?})", *COUNTER, ctx.exc_return()).unwrap();
    if *COUNTER == 10 {
        debug::exit(EXIT_SUCCESS);
    }
    ExceptionReturn::ThreadMsp
}

#[exception]
fn PendSV(ctx: ExceptionContext) -> ExceptionReturn {
    ctx.exc_return()
}
