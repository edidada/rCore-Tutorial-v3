pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != 1 || !crate::loader::valid_user_range(crate::task::current_id(), buf as usize, len) {
        return -1;
    }
    if len == 0 { return 0; }
    for byte in unsafe { core::slice::from_raw_parts(buf, len) } {
        crate::sbi::console_putchar(*byte as usize);
    }
    len as isize
}
