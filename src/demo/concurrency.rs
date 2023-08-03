use crate::{
    println,
    task::{simple_executor::SimpleExecutor, Task},
};

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
