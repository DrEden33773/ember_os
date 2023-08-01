pub mod cpu_exceptions;
pub mod double_fault;

#[inline]
pub fn run_demos() {
    // cpu_exceptions::invoke_breakpoint_exception();
    // double_fault::trigger_page_fault();
}
