#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod exit;
pub mod serial;
pub mod test_framework;
#[cfg(test)]
pub mod tests;
pub mod vga_buffer;

use core::panic::PanicInfo;
#[cfg(test)]
use test_framework::Testable;

/// Entry / Main
#[no_mangle]
pub(crate) extern "C" fn _start() -> ! {
    println!(" -*-*-*- My ROS -*-*-*- ");
    println!();
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
pub(crate) fn test_runner(tests: &[&dyn Testable]) {
    use crate::exit::{exit_qemu, QemuExitCode};
    serial_println!();
    serial_println!("Running {} tests", tests.len());
    serial_println!();
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(not(test))]
#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::exit::{exit_qemu, QemuExitCode};
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
