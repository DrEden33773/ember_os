use crate::{local_log_ln, println};
use alloc::{boxed::Box, vec};
use async_recursion::async_recursion;

pub async fn show_fib(n: usize) {
  #[async_recursion]
  async fn fib(n: usize) -> usize {
    match n {
      0 | 1 => 1,
      _ => fib(n - 1).await + fib(n - 2).await,
    }
  }
  let mut vec = vec![];
  for i in 0..=n {
    vec.push(fib(i).await);
  }
  local_log_ln!("Fibonacci from .0 to .{} ...", n);
  println!("Fibonacci := {:?}\n", vec);
}

pub async fn show_pi() {
  #[async_recursion]
  async fn pi_quarter(n: usize) -> f64 {
    match n {
      0 => 1.0,
      _ => {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        sign / (2 * n + 1) as f64 + pi_quarter(n - 1).await
      }
    }
  }
  let mut curr_pi = 0.0;
  const STEPS: usize = 1000;
  local_log_ln!("Calculating `PI` in {} steps ...", STEPS);
  for i in 0..STEPS {
    curr_pi = pi_quarter(i).await;
  }
  curr_pi *= 4.0;
  println!("`PI` = {}", curr_pi);
}
