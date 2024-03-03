use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
  pub static ref SERIAL1: Mutex<SerialPort> = {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
  };
}

pub fn safe_print(args: ::core::fmt::Arguments) {
  use core::fmt::Write;
  use x86_64::instructions::interrupts;

  // access SERIAL1 without being interrupted by signals
  interrupts::without_interrupts(|| {
    SERIAL1
      .lock()
      .write_fmt(args)
      .expect("printing to serial failed!");
  });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::safe_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
