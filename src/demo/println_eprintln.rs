use crate::{eprintln, local_log_ln, println};

#[inline]
pub fn show_color_diff() {
    eprintln!("`eprintln` color");
    println!("`println` color");
    local_log_ln!("`local_log_ln` color");
    println!();
}
