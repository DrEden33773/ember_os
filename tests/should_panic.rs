#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_ros::{
  exit::{exit_qemu, QemuExitCode},
  serial_print, serial_println,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");
  exit_qemu(QemuExitCode::Failed);
  my_ros::hlt_loop()
}

fn should_fail() {
  serial_print!("\nshould_panic::should_fail ... ");
  assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]\n");
  exit_qemu(QemuExitCode::Success);
  my_ros::hlt_loop()
}
