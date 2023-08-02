#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    my_ros::hlt_loop()
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    my_ros::test_panic_handler(info)
}
