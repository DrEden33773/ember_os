#[inline]
pub fn trigger_page_fault() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
}
