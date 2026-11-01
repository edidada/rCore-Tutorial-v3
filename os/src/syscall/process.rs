use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
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
