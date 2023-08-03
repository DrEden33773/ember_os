use crate::{
    println,
    task::{executor::Executor, simple_executor::SimpleExecutor, Task},
};
use alloc::{boxed::Box, vec};
use async_recursion::async_recursion;

#[inline]
pub fn async_task() {
    async fn async_number() -> u32 {
        0
    }
    async fn example_task() {
        println!("Async number: {}", async_number().await);
    }
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
}

#[inline]
pub fn fibonacci_demo() {
    #[async_recursion]
    async fn fib(n: usize) -> usize {
        match n {
            0 | 1 => 1,
            _ => fib(n - 1).await + fib(n - 2).await,
        }
    }
    async fn show_fib(n: usize) {
        let mut vec = vec![];
        for i in 0..=n {
            vec.push(fib(i).await);
        }
        println!("Fibonacci from .0 to .{}:\n{:?}", n, vec);
    }
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(show_fib(20)));
    executor.run();
}
