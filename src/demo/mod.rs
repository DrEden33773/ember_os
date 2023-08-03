#![allow(dead_code)]

use crate::println;
use bootloader::BootInfo;

pub mod concurrency;
pub mod cpu_exceptions;
pub mod double_fault;
pub mod heap_allocation;
pub mod memory;
pub mod println_eprintln;

#[inline]
pub fn run_demos(boot_info: &'static BootInfo) {
    memory::show_map_of_tables(boot_info);
    heap_allocation::create_box();
    heap_allocation::create_vec();
    heap_allocation::create_reference_counted_vec();
    println_eprintln::show_color_diff();
    concurrency::fibonacci_demo();
    println!();
}
