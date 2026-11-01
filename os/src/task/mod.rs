mod context;
mod switch;
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use lazy_static::*;
use switch::__switch;
use task::TaskControlBlock;
pub use task::TaskStatus;
use crate::sync::UPSafeCell;

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [
            TaskControlBlock {
                task_cx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit,
                priority: 16, pass: 0, started: 0, finished: 0, slices: 0
            };
            MAX_APP_NUM
        ];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })},
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        task0.started = crate::timer::get_time_us();
        task0.slices += 1;
        task0.pass += (1u128 << 64) / task0.priority as u128;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(
                &mut _unused as *mut TaskContext,
                next_task_cx_ptr,
            );
        }
        panic!("unreachable in run_first_task!");
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let task = &mut inner.tasks[current];
        task.task_status = TaskStatus::Exited;
        task.finished = crate::timer::get_time_us();
        println!("TASK DONE id={} priority={} slices={}", current, task.priority, task.slices);
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        (0..self.num_app)
            .filter(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
            .min_by_key(|id| inner.tasks[*id].pass)
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            let task = &mut inner.tasks[next];
            task.task_status = TaskStatus::Running;
            if task.started == 0 { task.started = crate::timer::get_time_us(); }
            task.slices += 1;
            task.pass += ((1u128 << 64) / task.priority as u128).max(1);
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(
                    current_task_cx_ptr,
                    next_task_cx_ptr,
                );
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            crate::sbi::shutdown();
        }
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
pub const MAX_SYSCALL_NUM: usize = 500;
static COUNTS: UPSafeCell<[[usize; MAX_SYSCALL_NUM]; MAX_APP_NUM]> = unsafe {
    UPSafeCell::new([[0; MAX_SYSCALL_NUM]; MAX_APP_NUM])
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallInfo { pub id: usize, pub times: usize }
#[repr(C)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}
pub fn current_id() -> usize { TASK_MANAGER.inner.exclusive_access().current_task }
pub fn count_syscall(number: usize) {
    if number < MAX_SYSCALL_NUM { COUNTS.exclusive_access()[current_id()][number] += 1; }
}
pub fn set_priority(priority: isize) -> isize {
    if priority < 2 { return -1; }
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    inner.tasks[current].priority = priority as usize;
    priority
}
pub fn task_info(id: usize, target: *mut TaskInfo) -> isize {
    let current = current_id();
    if id >= TASK_MANAGER.num_app || target as usize % core::mem::align_of::<TaskInfo>() != 0
        || !crate::loader::valid_user_range(current, target as usize, core::mem::size_of::<TaskInfo>()) {
        return -1;
    }
    let inner = TASK_MANAGER.inner.exclusive_access();
    let task = &inner.tasks[id];
    let counts = COUNTS.exclusive_access();
    // 直接分字段写入，不在 8KiB 内核栈上创建大型临时 TaskInfo。
    let info = unsafe { &mut *target };
    info.id = id;
    info.status = task.task_status;
    for number in 0..MAX_SYSCALL_NUM {
        info.call[number] = SyscallInfo { id: number, times: counts[id][number] };
    }
    info.time = if task.started == 0 { 0 } else {
        (if task.finished == 0 { crate::timer::get_time_us() } else { task.finished })
            .saturating_sub(task.started) / 1000
    };
    0
}
