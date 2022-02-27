// Turn off the standard library
#![no_std]
// Don't use the normal entry point chain
#![no_main]

// Imports
use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}