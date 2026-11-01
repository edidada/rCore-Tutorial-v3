#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use core::arch::asm;
#[repr(C)]
#[derive(Copy, Clone)]
struct Call { id: usize, times: usize }
#[repr(C)]
struct Info { id: usize, status: usize, call: [Call; 500], time: usize }
static mut INFO: Info = Info { id: 0, status: 0, call: [Call { id: 0, times: 0 }; 500], time: 0 };
fn task_info(id: usize, pointer: usize) -> isize {
    let result: isize;
    unsafe { core::arch::asm!("ecall", inlateout("a0") id => result, in("a1") pointer, in("a2") 0usize, in("a7") 410usize); }
    result
}
#[no_mangle]
fn main() -> i32 {
    println!("task_info test start");
    let pointer = unsafe { &mut INFO as *mut _ as usize };
    assert_eq!(task_info(0, pointer), 0);
    unsafe {
        assert_eq!(INFO.id, 0);
        assert_eq!(INFO.status, 2);
        assert_eq!(INFO.call[410].id, 410);
        assert_eq!(INFO.call[410].times, 1);
        assert!(INFO.call[64].times > 0);
    }
    let before = user_lib::get_time();
    while user_lib::get_time() - before < 20 { user_lib::yield_(); }
    assert_eq!(task_info(0, pointer), 0);
    unsafe { assert!(INFO.time >= 20); assert_eq!(INFO.call[410].times, 2); }
    assert_eq!(task_info(99, pointer), -1);
    assert_eq!(task_info(0, 0), -1);
    assert_eq!(task_info(0, usize::MAX - 4), -1);
    println!("Test task_info OK!");
    0
}
