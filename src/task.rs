use crate::SoftwareStackFrame;
use core::mem;
use core::ptr::write_volatile;
use cortex_m_rt::ExceptionFrame;

// Start in thumb mode
const PSR_DEFAULT: u32 = 0x0100_0000;

#[derive(Debug)]
enum TaskState {
    Created,
    Running,
    Suspended(*mut SoftwareStackFrame),
}

pub struct Task<'a> {
    state: TaskState,
    stack: &'a mut [u8],
}

impl<'a> Task<'a> {
    pub unsafe fn schedule_now(&mut self) {
        match self.state {
            TaskState::Created => {
                let stack_size = self.stack.len() as isize;
                let stack_ptr = self.stack.as_mut_ptr().offset(
                    stack_size
                        - (mem::size_of::<ExceptionFrame>() as isize)
                        - (mem::size_of::<SoftwareStackFrame>() as isize),
                );
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

    pub unsafe fn suspend(&mut self) {
        let stack_ptr = cortex_m::register::psp::read() as *mut SoftwareStackFrame;
        self.state = TaskState::Suspended(stack_ptr);
    }
}

pub struct TaskBuilder {
    sw_frame: SoftwareStackFrame,
    ex_frame: ExceptionFrame,
    args: u8,
}

impl TaskBuilder {
    pub fn new() -> TaskBuilder {
        TaskBuilder {
            sw_frame: SoftwareStackFrame::default(),
            ex_frame: ExceptionFrame {
                r0: 0,
                r1: 0,
                r2: 0,
                r3: 0,
                r12: 0,
                pc: 0,
                lr: 0,
                xpsr: PSR_DEFAULT,
            },
            args: 0,
        }
    }

    pub fn pc(mut self, pc: u32) -> Self {
        self.ex_frame.pc = pc;
        self
    }

    pub fn lr(mut self, lr: u32) -> Self {
        self.ex_frame.lr = lr;
        self
    }

    pub fn arg<I: Into<u32>>(mut self, a: I) -> Self {
        let a = a.into();
        match self.args {
            0 => self.ex_frame.r0 = a,
            1 => self.ex_frame.r1 = a,
            2 => self.ex_frame.r2 = a,
            3 => self.ex_frame.r3 = a,
            _ => panic!("Too many arguments for task"),
        }
        self.args += 1;
        self
    }

    pub unsafe fn build<'a>(self, stack: &'a mut [u8]) -> Task<'a> {
        let len = stack.len() as isize;
        let ptr = stack.as_mut_ptr() as *mut u8;
        let ex_ptr = ptr.offset(len - mem::size_of::<ExceptionFrame>() as isize);
        let sw_ptr = ex_ptr.offset(-(mem::size_of::<SoftwareStackFrame>() as isize));
        write_volatile(sw_ptr as *mut SoftwareStackFrame, self.sw_frame);
        write_volatile(ex_ptr as *mut ExceptionFrame, self.ex_frame);

        Task {
            state: TaskState::Created,
            stack,
        }
    }
}

// TODO more arguments
#[macro_export]
macro_rules! task {
    ($stack:expr, $func:expr) => {
        unsafe {
            $crate::task::TaskBuilder::new()
                .pc($func as usize as u32)
                .build($stack)
        }
    };
}
