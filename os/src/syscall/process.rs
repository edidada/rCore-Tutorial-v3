use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    current_task,
    current_user_token,
    add_task,
};
use crate::mm::{
    translated_str,
    translated_refmut,
};
use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = match crate::mm::checked_user_string(token, path as usize) { Some(path) => path, None => return -1 };
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    if inner.children
        .iter()
        .find(|p| {pid == -1 || pid as usize == p.getpid()})
        .is_none() {
        return -1;
        // ---- release current PCB
    }
    let pair = inner.children
        .iter()
        .enumerate()
        .find(|(_, p)| {
            // ++++ temporarily access child PCB lock exclusively
            p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB
        });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily access child TCB exclusively
        let exit_code = child.inner_exclusive_access().exit_code;
        // ++++ release child PCB
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let buffers = match crate::mm::checked_user_buffer(crate::task::current_user_token(), ts as usize, 16, true) {
        Some(buffers) => buffers, None => return -1,
    };
    let us = get_time_us();
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(us / 1_000_000).to_ne_bytes());
    bytes[8..].copy_from_slice(&(us % 1_000_000).to_ne_bytes());
    let mut offset = 0;
    for buffer in buffers { let len = buffer.len(); buffer.copy_from_slice(&bytes[offset..offset + len]); offset += len; }
    0
}

pub fn sys_spawn(path: *const u8) -> isize {
    let path = match crate::mm::checked_user_string(current_user_token(), path as usize) { Some(path) => path, None => return -1 };
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let parent = current_task().unwrap();
        let child = parent.spawn(data);
        let pid = child.pid.0;
        add_task(child);
        pid as isize
    } else { -1 }
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    current_task().unwrap().inner_exclusive_access().memory_set.mmap(start, len, prot)
}
pub fn sys_munmap(start: usize, len: usize) -> isize {
    current_task().unwrap().inner_exclusive_access().memory_set.munmap(start, len)
}
