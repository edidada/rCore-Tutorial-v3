const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != FD_STDOUT || !crate::batch::readable_user_range(buf as usize, len) {
        return -1;
    }
    if len == 0 { return 0; }
    // 验证地址后才构造切片；write 支持任意字节，不要求 UTF-8。
    let bytes = unsafe { core::slice::from_raw_parts(buf, len) };
    for byte in bytes { crate::sbi::console_putchar(*byte as usize); }
    len as isize
}
