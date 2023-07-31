#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_ros::{
    exit::{exit_qemu, QemuExitCode},
    serial_println,
    test_framework::ShouldPanicTestable,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn ShouldPanicTestable]) {
    serial_println!("\nRunning {} tests\n", tests.len());
    for test in tests {
        test.run();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    serial_println!();
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[test_case]
fn should_fail() {
    assert_eq!(0, 1);
}
