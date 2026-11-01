#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use user_lib::{mmap, munmap};
fn raw_write(pointer: usize, len: usize) -> isize {
    let result: isize;
    unsafe { core::arch::asm!("ecall", inlateout("a0") 1usize => result,
        in("a1") pointer, in("a2") len, in("a7") 64usize); }
    result
}
#[no_mangle]
fn main() -> i32 {
    let base = 0x11000000usize;
    assert_eq!(mmap(base, 3 * 4096, 3), 0);
    unsafe {
        (base as *mut u8).write_volatile(0x5a);
        ((base + 8192) as *mut u8).write_volatile(0x6b);
    }
    assert_eq!(munmap(base + 4096, 4096), 0);
    assert_eq!(munmap(base, 3 * 4096), -1);
    unsafe {
        assert_eq!((base as *const u8).read_volatile(), 0x5a);
        assert_eq!(((base + 8192) as *const u8).read_volatile(), 0x6b);
    }
    assert_eq!(mmap(base + 4096, 4096, 3), 0);
    assert_eq!(munmap(base, 3 * 4096), 0);
    assert_eq!(mmap(usize::MAX - 4095, 8192, 3), -1);
    assert_eq!(raw_write(usize::MAX, 2), -1);
    assert_eq!(mmap(base, 4096, 3), 0);
    assert_eq!(raw_write(base + 4090, 10), -1);
    assert_eq!(munmap(base, 4096), 0);
    println!("Test mapping boundaries OK!");
    0
}
