use crate::{serial_print, serial_println};

pub trait Testable {
  fn run(&self);
}

impl<T: Fn()> Testable for T {
  fn run(&self) {
    serial_print!("{} ... ", core::any::type_name::<T>());
    self();
    serial_println!("[ok]");
  }
}
