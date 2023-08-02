use crate::println;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

pub fn create_box() {
    let heap_value = Box::new(41);
    println!("`heap_value` at {:p}", heap_value);
}

pub fn create_vec() {
    let vec = (0..500).collect::<Vec<_>>();
    println!("`vec` at {:p}", vec.as_slice());
}

pub fn create_reference_counted_vec() {
    let rc_vec = Rc::new(vec![1, 2, 3]);
    let cloned_ref = rc_vec.clone();
    println!("Created `vec![1, 2, 3]` via `Rc`");
    println!(
        "Current reference count is {}",
        Rc::strong_count(&cloned_ref)
    );
    core::mem::drop(rc_vec);
    println!("Reference count is {} now", Rc::strong_count(&cloned_ref));
}
