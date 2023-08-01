#[inline]
pub fn invoke_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
