use crate::println;
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
    println!("Fibonacci from .0 to .{}:\n", n);
    println!("{:?}\n", vec);
}

pub async fn show_pi() {
    #[async_recursion]
    async fn pi(n: usize) -> f64 {
        match n {
            0 => 4.0,
            _ => {
                let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
                sign / (2 * n + 1) as f64 + pi(n - 1).await
            }
        }
    }
    let mut curr_pi = 0.0;
    const STEPS: usize = 100;
    println!("Calculating `PI` in {STEPS} steps:\n");
    for i in 0..STEPS {
        curr_pi = pi(i).await;
    }
    println!("PI = {}\n", curr_pi);
}
