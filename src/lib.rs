#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod allocator;
pub mod demo;
pub mod exit;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod prelude;
pub mod serial;
pub mod test_framework;
pub mod vga_buffer;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use exit::{exit_qemu, QemuExitCode};
use test_framework::Testable;

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
  init();
  test_main();
  hlt_loop();
}

pub fn test_runner(tests: &[&dyn Testable]) {
  serial_println!("\nRunning {} tests\n", tests.len());
  for test in tests {
    test.run();
  }
  serial_println!();
  exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  hlt_loop()
}

pub fn hlt_loop() -> ! {
  loop {
    x86_64::instructions::hlt()
  }
}

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}

pub fn init() {
  // gdt(tss) init
  gdt::init();
  // idt init
  interrupts::init_idt();
  // PIC init
  unsafe { interrupts::PICS.lock().initialize() };
  // enable listening on PIC
  x86_64::instructions::interrupts::enable();
}
