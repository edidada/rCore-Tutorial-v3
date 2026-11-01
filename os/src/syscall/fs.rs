use crate::mm::translated_byte_buffer;
use crate::task::{current_user_token, suspend_current_and_run_next};
use crate::sbi::console_getchar;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != 1 { return -1; }
    let buffers = match crate::mm::checked_user_buffer(crate::task::current_user_token(), buf as usize, len, false) {
        Some(buffers) => buffers, None => return -1,
    };
    for buffer in buffers { for byte in buffer { crate::sbi::console_putchar(*byte as usize); } }
    len as isize
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Only support len = 1 in sys_read!");
            let mut c: usize;
            loop {
                c = console_getchar();
                if c == 0 || c == usize::MAX {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            let mut buffers = translated_byte_buffer(current_user_token(), buf, len);
            unsafe { buffers[0].as_mut_ptr().write_volatile(ch); }
            1
        }
        _ => {
            panic!("Unsupported fd in sys_read!");
        }
    }
}