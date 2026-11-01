pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != 1 { return -1; }
    let buffers = match crate::mm::checked_user_buffer(crate::task::current_user_token(), buf as usize, len, false) {
        Some(buffers) => buffers, None => return -1,
    };
    for buffer in buffers { for byte in buffer { crate::sbi::console_putchar(*byte as usize); } }
    len as isize
}
