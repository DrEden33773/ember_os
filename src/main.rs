#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_ros::{demo, eprintln, println};

entry_point!(main);

/// Entry / Main
#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    println!(" -*-*-*- My ROS -*-*-*- \n");
    let mut executor = my_ros::init(boot_info);

    #[cfg(test)]
    test_main();

    println!(" ------- .Demos ------- \n");
    demo::run_demos(boot_info);

    println!(" >>>>>>> .Shell <<<<<<< \n");
    executor.run();

    my_ros::hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);
    my_ros::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_ros::test_panic_handler(info)
}

#[test_case]
fn test_framework_check() {
    eprintln!("Make sure the user defined test framework works!");
}
