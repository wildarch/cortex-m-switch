#![no_main]
#![no_std]

extern crate panic_semihosting;

use core::mem;
use core::ptr::read_volatile;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::register::psp;
use cortex_m::Peripherals;
use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprint, hprintln,
};
use cortex_m_switch::{exception, svc, ExceptionReturn};

const PSR_DEFAULT: u32 = 0x2100_0000;

#[repr(C)]
#[derive(Default)]
struct SoftwareStackFrame {
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
}

const STACK_SIZE: usize = 1024;
const STACK_CANARY_VALUE: u32 = 0xDEADBEEF;

#[repr(C)]
struct Stack {
    canary: u32,
    data: [u8; STACK_SIZE],
    // This only holds when the task is started,
    // later frames will have data after the frames.
    sw_stack_frame: SoftwareStackFrame,
    hw_stack_frame: ExceptionFrame,
}

impl Stack {
    pub fn is_overflowed(&self) -> bool {
        self.canary != STACK_CANARY_VALUE
    }
}

#[derive(Debug)]
enum TaskState {
    Created,
    Running,
    Suspended(*mut SoftwareStackFrame),
}

struct Task {
    state: TaskState,
    stack: Stack,
}

impl Task {
    pub fn new(func: fn() -> !) -> Task {
        Task {
            state: TaskState::Created,
            stack: Stack {
                canary: STACK_CANARY_VALUE,
                data: [0; STACK_SIZE],
                sw_stack_frame: SoftwareStackFrame::default(),
                hw_stack_frame: ExceptionFrame {
                    r0: 0,
                    r1: 0,
                    r2: 0,
                    r3: 0,
                    r12: 0,
                    pc: func as usize as u32, // Clippy will warn about direct cast
                    // TODO point to task cleanup function
                    lr: 0,
                    xpsr: PSR_DEFAULT,
                },
            },
        }
    }

    pub unsafe fn schedule_now(&mut self) {
        match self.state {
            TaskState::Created => {
                let stack_ptr = &mut self.stack.sw_stack_frame as *mut SoftwareStackFrame;
                cortex_m::register::psp::write(stack_ptr as u32);
                self.state = TaskState::Running;
            }
            TaskState::Suspended(stack_ptr) => {
                cortex_m::register::psp::write(stack_ptr as u32);
                self.state = TaskState::Running;
            }
            TaskState::Running => panic!("Task was left in state Running!"),
        }
    }

    pub unsafe fn save_context(&mut self) {
        let stack_ptr = cortex_m::register::psp::read() as *mut SoftwareStackFrame;
        if self.stack.is_overflowed() {
            panic!("Stack overflow detected!");
        }
        self.state = TaskState::Suspended(stack_ptr);
    }
}

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

const DELAY_CYCLES: u32 = 10_000_000;

fn print_loop() -> ! {
    loop {
        unsafe { svc!(0) };
        hprint!("O").unwrap();
        cortex_m::asm::delay(DELAY_CYCLES);
        unsafe { svc!(2) };
    }
}

fn stack_filler() -> ! {
    // Bytes left for function calls, local variables, exception frame etc.
    const BREATHING_ROOM: usize = 256;
    let mut data = [0u16; (STACK_SIZE - BREATHING_ROOM) / 2];

    loop {
        for (i, entry) in data.iter_mut().enumerate() {
            *entry = i as u16;
        }
        for entry in data.iter().rev() {
            hprint!("{}, ", entry).unwrap();
            cortex_m::asm::delay(DELAY_CYCLES);
        }
    }
}

const NROF_TASKS: usize = 2;

#[exception]
fn SysTick(exc: ExceptionReturn) -> ExceptionReturn {
    static mut TASK_INDEX: usize = 0;
    static mut TASKS: [Option<Task>; NROF_TASKS] = [None, None];

    hprint!("\n[{}]: Tick ({:?}): ", *TASK_INDEX, exc).unwrap();

    if *TASK_INDEX == 10 {
        hprintln!("Done!").unwrap();
        debug::exit(EXIT_SUCCESS);
    }

    if let Some(ref mut task) = TASKS[*TASK_INDEX % NROF_TASKS] {
        if let TaskState::Running = task.state {
            unsafe { task.save_context() };
            *TASK_INDEX = *TASK_INDEX + 1;
        } else {
            panic!("Task was not running! {:?}", task.state);
        }
    } else {
        TASKS[0] = Some(Task::new(print_loop));
        TASKS[1] = Some(Task::new(stack_filler));
    }

    if let Some(ref mut task) = TASKS[*TASK_INDEX % NROF_TASKS] {
        unsafe { task.schedule_now() };
    }

    hprintln!("Scheduled!").unwrap();

    ExceptionReturn::ThreadPsp
}

#[exception]
fn SVCall(exc: ExceptionReturn) -> ExceptionReturn {
    hprintln!("System call!").unwrap();
    let stack_ptr = psp::read();
    let stack_ptr = stack_ptr + mem::size_of::<SoftwareStackFrame>() as u32;
    let stack_ptr = stack_ptr as *const ExceptionFrame;
    let frame = unsafe { read_volatile(stack_ptr) };
    let svc_num = unsafe { read_volatile((frame.pc - 2) as *const u8) };
    hprintln!("System call number: {}", svc_num).unwrap();
    exc
}
