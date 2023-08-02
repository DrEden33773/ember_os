#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_ros::{
  demo, hlt_loop,
  memory::{self, BootInfoFrameAllocator},
  println,
};
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

/// Entry / Main
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
  println!(" -*-*-*- My ROS -*-*-*- \n");

  my_ros::init();

  #[cfg(test)]
  test_main();

  println!("Successfully loaded with NO crashes!\n");
  println!("Now, step into shell.\n");
  println!(" >>>>>>> .Shell <<<<<<< \n");

  demo::memory::show_map_of_tables(boot_info);
  // demo::memory::create_new_map_of_tables(boot_info);

  hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  my_ros::test_panic_handler(info)
}
