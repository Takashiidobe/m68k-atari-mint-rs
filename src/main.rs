#![no_std]
#![no_main]

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

use alloc::vec;

extern crate alloc;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    unsafe fn printf(fmt: *const u8, ...) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let fmt = b"%s\r\n\0";
    let msg = b"Hello from Rust on MiNT (mintlib)!\0";
    unsafe {
        printf(fmt.as_ptr(), msg.as_ptr());
    }
    let nums = vec![1, 2, 3];
    for num in nums {
        unsafe {
            printf(b"%d\r\n\0".as_ptr(), num);
        }
    }
    0
}
