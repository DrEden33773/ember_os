#![allow(dead_code)]

pub mod cpu_exceptions;
pub mod double_fault;
pub mod heap_allocation;
pub mod memory;

#[inline]
#[deprecated = "redundant"]
pub fn run_demos() {
    cpu_exceptions::invoke_breakpoint_exception();
    double_fault::trigger_page_fault();
}
