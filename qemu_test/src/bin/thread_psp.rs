#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm::delay;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m_switch::{exception, svc, task, ExceptionContext, ExceptionReturn, Task};

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}

fn print_task() {
    loop {
        hprintln!("Hello, world!").unwrap();
        unsafe { svc!(0) };
        for _ in 0..100 {
            delay(10_000_000);
        }
    }
}

fn print_task2() {
    loop {
        hprintln!("Hallo, chinees?").unwrap();
        unsafe { svc!(1) };
        for _ in 0..100 {
            delay(10_000_000);
        }
    }
}

#[exception]
fn SysTick(ctx: ExceptionContext) -> ExceptionReturn {
    static mut STACK1: [u8; 256] = [0; 256];
    static mut STACK2: [u8; 256] = [0; 256];
    static mut TASKS: Option<[Task; 2]> = None;
    static mut COUNTER: usize = 0;

    hprintln!("Scheduler tick {}", COUNTER).unwrap();

    if let Some(tasks) = TASKS {
        // Suspend the running task
        let task_index = *COUNTER % tasks.len();
        unsafe { tasks[task_index].suspend() };

        *COUNTER += 1;

        // Start the next task
        let task_index = *COUNTER % tasks.len();
        unsafe { tasks[task_index].schedule_now() };
    } else {
        // Initialize tasks
        let mut tasks = [
            task!(&mut *STACK1, print_task),
            task!(&mut *STACK2, print_task2),
        ];
        unsafe { tasks[0].schedule_now() };
        *TASKS = Some(tasks);
    }

    hprintln!("Tick ({:?})", ctx.exc_return()).unwrap();
    ExceptionReturn::ThreadPsp
}

#[exception]
fn SVCall(ctx: ExceptionContext) -> ExceptionReturn {
    if let Some(num) = ctx.svc_num() {
        hprintln!("System call number: {}", num).unwrap();
    } else {
        hprintln!("Could not retrieve System call number!").unwrap();
    }
    ctx.exc_return()
}
