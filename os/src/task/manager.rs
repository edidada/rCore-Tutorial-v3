use crate::sync::UPSafeCell;
use super::TaskControlBlock;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new(), }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = unsafe {
        UPSafeCell::new(TaskManager::new())
    };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASKS.exclusive_access().insert(task.pid.0, Arc::downgrade(&task));
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
lazy_static! {
    static ref TASKS: UPSafeCell<alloc::collections::BTreeMap<usize, alloc::sync::Weak<TaskControlBlock>>> = unsafe {
        UPSafeCell::new(alloc::collections::BTreeMap::new())
    };
}
pub fn find_task(pid: usize) -> Option<Arc<TaskControlBlock>> {
    TASKS.exclusive_access().get(&pid).and_then(|task| task.upgrade())
}
