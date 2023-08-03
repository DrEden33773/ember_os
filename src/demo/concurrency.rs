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
    println!("Fibonacci from .0 to .{}:\n{:?}", n, vec);
}
