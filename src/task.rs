use crate::SoftwareStackFrame;
use arrayvec::{Array, ArrayVec};
use core::mem;
use core::ptr::write_volatile;
use cortex_m_rt::ExceptionFrame;

// Start in thumb mode
const PSR_DEFAULT: u32 = 0x0100_0000;

/// The stack type is full descending (start from high address, down to low address).
/// See also: [ARM Reference](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0068b/Cacbgchh.html).
///
/// When the stack is first created, it is laid out like this:
/// ```
/// == Stack memory begins here ==
///
///         ... Empty ...
///
/// [SoftwareStackFrame]   <-- Stack pointer (type *const SoftwareStackFrame)
/// [ExceptionFrame]
/// ==  Stack memory ends here  ==
/// ```
///
/// Once the task has started (and pushed some data to the stack), it looks like this:
/// ```
/// == Stack memory begins here ==
///
///         ... Empty ...
///
/// [SoftwareStackFrame]   <-- Stack pointer
/// [ExceptionFrame]
/// [Stack item 1]
/// [Stack item 2]
///      ...
/// [Stack item n]
/// ==  Stack memory ends here  ==
/// ```
pub struct Stack<A: Array> {
    data: ArrayVec<A>,
}

impl<A: Array<Item = u8>> Stack<A> {
    pub fn new() -> Stack<A> {
        Stack {
            data: ArrayVec::default(),
        }
    }

    pub fn init(&mut self, sw_frame: SoftwareStackFrame, ex_frame: ExceptionFrame) {
        let len = self.size() as isize;
        let ptr = self.data.as_mut_ptr() as *mut u8;
        unsafe {
            let ex_ptr = ptr.offset(len - mem::size_of::<ExceptionFrame>() as isize);
            let sw_ptr = ex_ptr.offset(-(mem::size_of::<SoftwareStackFrame>() as isize));
            write_volatile(sw_ptr as *mut SoftwareStackFrame, sw_frame);
            write_volatile(ex_ptr as *mut ExceptionFrame, ex_frame);
        }
    }

    pub fn size(&self) -> usize {
        self.data.capacity()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

#[derive(Debug)]
enum TaskState {
    Created,
    Running,
    Suspended(*mut SoftwareStackFrame),
}

pub struct Task<A: Array> {
    state: TaskState,
    stack: Stack<A>,
}

impl<A: Array<Item = u8>> Task<A> {
    pub unsafe fn schedule_now(&mut self) {
        match self.state {
            TaskState::Created => {
                let stack_size = self.stack.size() as isize;
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

    pub unsafe fn build<A: Array<Item = u8>>(self, mut stack: Stack<A>) -> Task<A> {
        stack.init(self.sw_frame, self.ex_frame);
        Task {
            state: TaskState::Created,
            stack,
        }
    }
}

// TODO more arguments
#[macro_export]
macro_rules! task {
    ($size:expr, $func:expr) => {
        unsafe {
            $crate::task::TaskBuilder::new()
                .pc($func as usize as u32)
                .build($crate::task::Stack::<[_; $size]>::new())
        }
    };
}
