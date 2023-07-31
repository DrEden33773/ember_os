#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[cfg(test)]
pub mod tests;

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
    println!(" -*-*-*- My ROS -*-*-*- ");
    println!();
    #[cfg(test)]
    test_main();
    loop {}
}
