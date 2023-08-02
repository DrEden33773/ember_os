#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_ros::{demo, hlt_loop, println};

entry_point!(main);

/// Entry / Main
#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    println!(" -*-*-*- My ROS -*-*-*- \n");

    my_ros::init(boot_info);

    #[cfg(test)]
    test_main();

    println!("Successfully loaded with NO crashes!\n");
    println!("Now, step into shell.\n");
    println!(" >>>>>>> .Shell <<<<<<< \n");

    demo::memory::show_map_of_tables(boot_info);
    demo::heap_allocation::create_box();
    demo::heap_allocation::create_vec();
    demo::heap_allocation::create_reference_counted_vec();

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

#[test_case]
fn test_framework_check() {
    println!("Make sure the user defined test framework works!");
}
