use super::TaskContext;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub priority: usize,
    pub pass: u128,
    pub started: usize,
    pub finished: usize,
    pub slices: usize,
}

#[repr(usize)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}