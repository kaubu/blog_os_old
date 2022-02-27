// Turn off the standard library
#![no_std]
// Don't use the normal entry point chain
#![no_main]
// Enable custom test frameworks
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// Imports
use core::panic::PanicInfo;
mod vga_buffer;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}