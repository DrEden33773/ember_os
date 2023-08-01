#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_ros::{hlt_loop, println};

/// Entry / Main
#[no_mangle]
pub(crate) extern "C" fn _start() -> ! {
    println!(" -*-*-*- My ROS -*-*-*- \n");

    my_ros::init();

    #[cfg(test)]
    test_main();

    println!("Successfully loaded with NO crashes!\n");
    println!("Now, step into shell.\n");

    println!(" >>>>>>> .Shell <<<<<<< \n");

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
