#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

pub mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(test)]
pub(crate) fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub(crate) extern "C" fn _start() -> ! {
    println!("Hello, world!");
    loop {}
}
