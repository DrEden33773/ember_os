#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use my_ros::{demo, println};

/// Entry / Main
#[no_mangle]
pub(crate) extern "C" fn _start() -> ! {
    println!(" -*-*-*- My ROS -*-*-*- \n");

    my_ros::init();

    #[cfg(test)]
    test_main();

    demo::run_demos();

    println!(" >>>>>>> .Shell <<<<<<< \n");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_ros::test_panic_handler(info)
}
