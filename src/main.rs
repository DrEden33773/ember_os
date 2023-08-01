#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_ros::{
    hlt_loop,
    memory::{self},
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

    show_map_of_tables(boot_info);

    hlt_loop()
}

fn show_map_of_tables(boot_info: &BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
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
