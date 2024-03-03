use crate::demo::concurrency;
use alloc::boxed::Box;
use core::{
  future::Future,
  pin::Pin,
  sync::atomic::{AtomicU64, Ordering},
  task::{Context, Poll},
};

pub mod executor;
pub mod keyboard;
pub mod simple_executor;

cfg_if::cfg_if! {
  if #[cfg(feature = "use_SimpleExecutor")] {
    type UsedExecutor = simple_executor::SimpleExecutor;
  } else {
    type UsedExecutor = executor::Executor;
  }
}

pub struct Task {
  id: TaskId,
  future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
  pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
    Task {
      id: TaskId::new(),
      future: Box::pin(future),
    }
  }

  fn poll(&mut self, context: &mut Context) -> Poll<()> {
    self.future.as_mut().poll(context)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
  fn new() -> Self {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
  }
}

impl UsedExecutor {
  fn spawn_hardware_task(&mut self) {
    self.spawn(Task::new(keyboard::print_keypresses()));
  }

  fn spawn_long_computation_demos(&mut self) {
    self.spawn(Task::new(concurrency::show_fib(20)));
    self.spawn(Task::new(concurrency::cached_show_fib(60)));
    self.spawn(Task::new(concurrency::show_pi()));
  }
}

pub fn init_demos_only() -> UsedExecutor {
  let mut executor = UsedExecutor::new();
  executor.spawn_long_computation_demos();
  executor
}

pub fn init_hardwares_only() -> UsedExecutor {
  let mut executor = UsedExecutor::new();
  executor.spawn_hardware_task();
  executor
}

#[deprecated = "It will be much clear if you could separate `hardware-IO-needless demos` and `hardware-IO-related looping tasks`"]
pub fn init() -> UsedExecutor {
  let mut executor = UsedExecutor::new();
  executor.spawn_hardware_task();
  executor.spawn_long_computation_demos();
  executor
}
