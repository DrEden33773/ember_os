#![allow(dead_code, unused_import_braces, unused_imports)]

use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use lazy_static::lazy_static;
use spin::mutex::Mutex;

pub mod keyboard;
pub mod simple_executor;

use simple_executor::SimpleExecutor;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

pub fn init() -> SimpleExecutor {
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor
}
