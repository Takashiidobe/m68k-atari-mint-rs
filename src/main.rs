#![no_std]
#![no_main]

unsafe extern "C" {
    unsafe fn printf(fmt: *const u8, ...) -> i32;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let fmt = b"%s\r\n\0";
    let msg = b"Hello from Rust on MiNT (mintlib)!\0";
    unsafe {
        printf(fmt.as_ptr(), msg.as_ptr());
    }
    0
}
