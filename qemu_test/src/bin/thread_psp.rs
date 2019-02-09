#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm::delay;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m_switch::{exception, svc, svc_num, task, ExceptionReturn, Task};

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
fn SysTick(exc: ExceptionReturn) -> ExceptionReturn {
    static mut TASKS: Option<[Task<[u8; 256]>; 2]> = None;
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
        let mut tasks = [task!(256, print_task), task!(256, print_task2)];
        unsafe { tasks[0].schedule_now() };
        *TASKS = Some(tasks);
    }

    hprintln!("Tick ({:?})", exc).unwrap();
    ExceptionReturn::ThreadPsp
}

#[exception]
fn SVCall(exc: ExceptionReturn) -> ExceptionReturn {
    hprintln!("System call number: {}", svc_num().unwrap()).unwrap();
    exc
}
