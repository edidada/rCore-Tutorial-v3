use crate::mm::{
    UserBuffer,
    translated_byte_buffer,
    translated_refmut,
    translated_str,
};
use crate::task::{current_user_token, current_task};
use crate::fs::{make_pipe, OpenFlags, open_file};
use alloc::sync::Arc;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(
            UserBuffer::new(translated_byte_buffer(token, buf, len))
        ) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.read(
            UserBuffer::new(translated_byte_buffer(token, buf, len))
        ) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(
        path.as_str(),
        OpenFlags::from_bits(flags).unwrap()
    ) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.inner_exclusive_access();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}

pub fn sys_dup(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(Arc::clone(inner.fd_table[fd].as_ref().unwrap()));
    new_fd as isize
}
pub fn sys_fstat(fd: usize, pointer: usize) -> isize {
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    let file = match inner.fd_table.get(fd).and_then(|file| file.as_ref()) { Some(file) => file.clone(), None => return -1 };
    drop(inner);
    let (ino, links) = match file.metadata() { Some(value) => value, None => return -1 };
    let buffers = match crate::mm::checked_user_buffer(current_user_token(), pointer, 80, true) { Some(buffers) => buffers, None => return -1 };
    let mut bytes = [0u8; 80];
    bytes[8..16].copy_from_slice(&ino.to_ne_bytes());
    bytes[16..20].copy_from_slice(&0o100000u32.to_ne_bytes());
    bytes[20..24].copy_from_slice(&links.to_ne_bytes());
    let mut offset = 0;
    for buffer in buffers { let size = buffer.len(); buffer.copy_from_slice(&bytes[offset..offset + size]); offset += size; }
    0
}
pub fn sys_link(old: usize, new: usize) -> isize {
    let token = current_user_token();
    let old = match crate::mm::checked_user_string(token, old) { Some(path) => path, None => return -1 };
    let new = match crate::mm::checked_user_string(token, new) { Some(path) => path, None => return -1 };
    if crate::fs::ROOT_INODE.link(&old, &new) { 0 } else { -1 }
}
pub fn sys_unlink(path: usize) -> isize {
    let path = match crate::mm::checked_user_string(current_user_token(), path) { Some(path) => path, None => return -1 };
    if crate::fs::ROOT_INODE.unlink(&path) { 0 } else { -1 }
}
