#![no_std]
#![no_main]

// Declare just the C function(s) you need from mintlib:
unsafe extern "C" {
    // printf(const char*, ...)
    unsafe fn printf(fmt: *const core::ffi::c_char, ...) -> i32;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Export a C-style main so mintlib’s CRT can call us.
#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let fmt = b"%s\r\n\0";
    let msg = b"Hello from Rust on MiNT (mintlib)!\0";
    unsafe {
        printf(fmt.as_ptr(), msg.as_ptr());
    }
    0
}
