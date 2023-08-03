use crate::{eprintln, println};

#[inline]
pub fn show_color_diff() {
    eprintln!("Test eprintln");
    println!("Test println after eprintln");
}
