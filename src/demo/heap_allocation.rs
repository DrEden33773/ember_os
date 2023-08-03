use crate::{local_log_ln, println};
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

pub fn create_box() {
    let heap_value = Box::new(41);
    local_log_ln!("Called |> heap_value := Box::new(41)");
    println!("heap_value.addr = {:p}\n", heap_value);
}

pub fn create_vec() {
    let vec = (0..500).collect::<Vec<_>>();
    local_log_ln!("Called |> vec := (0..500).collect::<Vec<_>>()");
    println!("vec.addr = {:p}\n", vec.as_slice());
}

pub fn create_reference_counted_vec() {
    let rc_vec = Rc::new(vec![1, 2, 3]);
    local_log_ln!("Called |> rc_vec := Rc::new(vec![1, 2, 3])");

    let cloned_ref = rc_vec.clone();
    local_log_ln!("Called |> cloned_ref := rc_vec.clone()");

    println!("Reference.Count = {}", Rc::strong_count(&rc_vec));

    core::mem::drop(rc_vec);
    local_log_ln!("Called |> core::mem::drop(rc_vec);");

    println!("Reference.Count = {}\n", Rc::strong_count(&cloned_ref));
}
