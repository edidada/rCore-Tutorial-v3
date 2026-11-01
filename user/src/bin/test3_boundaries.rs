#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use user_lib::{mmap, munmap, write};
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
    assert_eq!(write(1, unsafe { core::slice::from_raw_parts(usize::MAX as *const u8, 2) }), -1);
    assert_eq!(mmap(base, 4096, 3), 0);
    assert_eq!(write(1, unsafe { core::slice::from_raw_parts((base + 4090) as *const u8, 10) }), -1);
    assert_eq!(munmap(base, 4096), 0);
    println!("Test mapping boundaries OK!");
    0
}
