use crate::{mm::checked_user_buffer, task::{current_task, current_user_token, find_task}};
use alloc::vec::Vec;
pub fn write(pid: usize, pointer: usize, len: usize) -> isize {
    let target = match find_task(pid) { Some(task) => task, None => return -1 };
    let inner = target.inner_exclusive_access();
    if inner.is_zombie() || inner.mailbox.len() >= 16 { return -1; }
    drop(inner);
    let len = len.min(256);
    if len == 0 { return 0; }
    let buffers = match checked_user_buffer(current_user_token(), pointer, len, false) {
        Some(buffers) => buffers, None => return -1,
    };
    let mut message = Vec::with_capacity(len);
    for buffer in buffers { message.extend_from_slice(buffer); }
    target.inner_exclusive_access().mailbox.push_back(message);
    len as isize
}
pub fn read(pointer: usize, len: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let message = match inner.mailbox.front() { Some(message) => message, None => return -1 };
    let len = len.min(message.len());
    if len == 0 { return 0; }
    let buffers = match checked_user_buffer(inner.memory_set.token(), pointer, len, true) {
        Some(buffers) => buffers, None => return -1,
    };
    let mut offset = 0;
    for buffer in buffers { let size = buffer.len(); buffer.copy_from_slice(&message[offset..offset + size]); offset += size; }
    inner.mailbox.pop_front();
    len as isize
}
