use alloc::boxed::Box;
use core::{
  fmt::{Debug, Display},
  ptr::NonNull,
};

pub mod iter;

#[derive(Default)]
pub struct ListNode<T: Default> {
  prev: Option<NonNull<ListNode<T>>>,
  next: Option<NonNull<ListNode<T>>>,
  value: T,
}

impl<T: Default> ListNode<T> {
  pub fn new(value: T) -> Self {
    Self {
      prev: None,
      next: None,
      value,
    }
  }
}

impl<T: Default + Display> Display for ListNode<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl<T: Default + Debug> Debug for ListNode<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("ListNode")
      .field("prev", &self.prev)
      .field("next", &self.next)
      .field("value", &self.value)
      .finish()
  }
}

impl<T: Default + PartialOrd> PartialOrd for ListNode<T> {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.value.partial_cmp(&other.value)
  }
}

impl<T: Default + Ord> Ord for ListNode<T> {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.value.cmp(&other.value)
  }
}

impl<T: Default + PartialEq> PartialEq for ListNode<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl<T: Default + Eq> Eq for ListNode<T> {}

pub struct LinkedList<T: Default> {
  head: NonNull<ListNode<T>>,
  tail: NonNull<ListNode<T>>,
  len: usize,
}

impl<T: Default> Drop for LinkedList<T> {
  fn drop(&mut self) {
    // 1. drop all nodes in `head.next .. tail`
    // 2. drop head and tail
    self.clear();
    let _ = unsafe { Box::from_raw(self.head.as_ptr()) };
    let _ = unsafe { Box::from_raw(self.tail.as_ptr()) };
  }
}

impl<T: Default + Display> Display for LinkedList<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, e) in self.iter().enumerate() {
      write!(f, "{}", e)?;
      if i != self.len - 1 {
        write!(f, ", ")?;
      }
    }
    write!(f, "]")?;
    Ok(())
  }
}

impl<T: Default + Debug> Debug for LinkedList<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("LinkedList")
      .field("head", &self.head)
      .field("tail", &self.tail)
      .field("len", &self.len)
      .finish()
  }
}

impl<T: Default + Clone> Clone for LinkedList<T> {
  fn clone(&self) -> Self {
    self.iter().cloned().collect::<Self>()
  }
}

impl<T: Default + PartialOrd> PartialOrd for LinkedList<T> {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    for (a, b) in self.iter().zip(other.iter()) {
      match a.partial_cmp(b) {
        Some(core::cmp::Ordering::Equal) => continue,
        x => return x,
      }
    }
    Some(self.len.cmp(&other.len))
  }
}

impl<T: Default + Ord> Ord for LinkedList<T> {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    for (a, b) in self.iter().zip(other.iter()) {
      match a.cmp(b) {
        core::cmp::Ordering::Equal => continue,
        x => return x,
      }
    }
    self.len.cmp(&other.len)
  }
}

impl<T: Default + PartialEq> PartialEq for LinkedList<T> {
  fn eq(&self, other: &Self) -> bool {
    for (a, b) in self.iter().zip(other.iter()) {
      if a != b {
        return false;
      }
    }
    true
  }
}

impl<T: Default + Eq> Eq for LinkedList<T> {}

impl<T: Default> Default for LinkedList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Default> LinkedList<T> {
  pub fn new() -> Self {
    let mut head = Box::<ListNode<T>>::default();
    let mut tail = Box::<ListNode<T>>::default();
    head.next = Some(unsafe { NonNull::new_unchecked(tail.as_mut()) });
    tail.prev = Some(unsafe { NonNull::new_unchecked(head.as_mut()) });

    Self {
      head: unsafe { NonNull::new_unchecked(Box::leak(head)) },
      tail: unsafe { NonNull::new_unchecked(Box::leak(tail)) },
      len: 0,
    }
  }
}

impl<T: Default> LinkedList<T> {
  pub fn push_front(&mut self, value: T) {
    let new_node = Box::new(ListNode::new(value));

    let mut new = unsafe { NonNull::new_unchecked(Box::leak(new_node)) };
    let mut head_next = unsafe { self.head.as_mut() }.next.unwrap();

    unsafe {
      new.as_mut().prev = Some(self.head);
      self.head.as_mut().next = Some(new);
      new.as_mut().next = Some(head_next);
      head_next.as_mut().prev = Some(new);
    };

    self.len += 1;
  }

  pub fn push_back(&mut self, value: T) {
    let new_node = Box::new(ListNode::new(value));

    let mut new = unsafe { NonNull::new_unchecked(Box::leak(new_node)) };
    let mut tail_prev = unsafe { self.tail.as_mut() }.prev.unwrap();

    unsafe {
      new.as_mut().prev = Some(tail_prev);
      tail_prev.as_mut().next = Some(new);
      new.as_mut().next = Some(self.tail);
      self.tail.as_mut().prev = Some(new);
    };

    self.len += 1;
  }

  pub fn push_nth(&mut self, value: T, n: usize) -> bool {
    if n > self.len {
      return false;
    }

    let new_node = Box::new(ListNode::new(value));
    let mut new = unsafe { NonNull::new_unchecked(Box::leak(new_node)) };

    let mut location = unsafe { self.head.as_mut() }.next.unwrap();
    unsafe {
      for _ in 0..n {
        location = location.as_mut().next.unwrap();
      }
    }
    let mut the_prev = unsafe { location.as_mut() }.prev.unwrap();

    unsafe {
      new.as_mut().prev = Some(the_prev);
      the_prev.as_mut().next = Some(new);
      new.as_mut().next = Some(location);
      location.as_mut().prev = Some(new);
    };

    self.len += 1;

    true
  }
}

impl<T: Default> LinkedList<T> {
  pub fn pop_front(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }

    let mut front = unsafe { self.head.as_mut() }.next.unwrap();
    let mut new_front = unsafe { front.as_mut() }.next.unwrap();

    unsafe {
      self.head.as_mut().next = Some(new_front);
      new_front.as_mut().prev = Some(self.head);
    };

    self.len -= 1;

    unsafe { Box::from_raw(front.as_ptr()) }.value.into()
  }

  pub fn pop_back(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }

    let mut back = unsafe { self.tail.as_mut() }.prev.unwrap();
    let mut new_back = unsafe { back.as_mut() }.prev.unwrap();

    unsafe {
      self.tail.as_mut().prev = Some(new_back);
      new_back.as_mut().next = Some(self.tail);
    };

    self.len -= 1;

    unsafe { Box::from_raw(back.as_ptr()) }.value.into()
  }

  pub fn pop_nth(&mut self, n: usize) -> Option<T> {
    if self.len == 0 || n >= self.len {
      return None;
    }

    let mut location = unsafe { self.head.as_mut() }.next.unwrap();
    unsafe {
      for _ in 0..n {
        location = location.as_mut().next.unwrap();
      }
    }

    let mut the_prev = unsafe { location.as_mut() }.prev.unwrap();
    let mut the_next = unsafe { location.as_mut() }.next.unwrap();

    unsafe {
      the_prev.as_mut().next = Some(the_next);
      the_next.as_mut().prev = Some(the_prev);
    };

    self.len -= 1;

    unsafe { Box::from_raw(location.as_ptr()) }.value.into()
  }

  pub fn clear(&mut self) {
    self.len = 0;
    unsafe {
      let mut curr = self.head.as_ref().next.unwrap();
      while curr.as_ptr() != self.tail.as_ptr() {
        let next = curr.as_ref().next.unwrap();
        let _ = Box::from_raw(curr.as_ptr()); // drop
        curr = next;
      }
      self.head.as_mut().next = Some(self.tail);
      self.tail.as_mut().prev = Some(self.head);
    };
  }
}

impl<T: Default> LinkedList<T> {
  pub fn first(&self) -> Option<&T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let front = &self.head.as_ref().next.unwrap();
      Some(&front.as_ref().value)
    }
  }

  pub fn last(&self) -> Option<&T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let back = &self.tail.as_ref().prev.unwrap();
      Some(&back.as_ref().value)
    }
  }

  pub fn get(&self, n: usize) -> Option<&T> {
    if n >= self.len {
      return None;
    }
    unsafe {
      let mut location = self.head.as_ref().next.unwrap();
      for _ in 0..n {
        location = location.as_ref().next.unwrap();
      }
      Some(&location.as_ref().value)
    }
  }
}

impl<T: Default> LinkedList<T> {
  pub fn first_mut(&mut self) -> Option<&mut T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let front = &mut self.head.as_mut().next.unwrap();
      Some(&mut front.as_mut().value)
    }
  }

  pub fn last_mut(&mut self) -> Option<&mut T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let back = &mut self.tail.as_mut().prev.unwrap();
      Some(&mut back.as_mut().value)
    }
  }

  pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
    if n >= self.len {
      return None;
    }
    unsafe {
      let mut location = self.head.as_mut().next.unwrap();
      for _ in 0..n {
        location = location.as_mut().next.unwrap();
      }
      Some(&mut location.as_mut().value)
    }
  }
}

impl<T: Default> LinkedList<T> {
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

#[cfg(test)]
mod test_linked_list {
  use alloc::format;
  use alloc::vec;
  use alloc::vec::Vec;

  use super::*;

  #[test_case]
  fn add_elements() {
    let mut list = LinkedList::new();
    for i in [1, 2, 3] {
      list.push_front(i);
    }
    for i in [1, 2, 3] {
      list.push_back(i);
    }
    assert_eq!(list.len(), 6);
  }

  #[test_case]
  fn pop_elements() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1]);
    list.push_nth(3, list.len());
    list.push_nth(2, list.len() - 1);
    list.pop_nth(2);
    list.pop_nth(2);
    let mut collected = vec![];
    for _ in 0..list.len() {
      collected.push(list.pop_front().unwrap());
    }
    assert_eq!(collected, [3, 2, 2, 3]);
  }

  #[test_case]
  fn change_element() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.first_mut().unwrap() = 4;
    *list.last_mut().unwrap() = 4;
    let mut collected = vec![];
    for _ in 0..list.len() {
      collected.push(list.pop_front().unwrap());
    }
    assert_eq!(collected, [4, 2, 1, 1, 2, 4]);
  }

  #[test_case]
  fn get_element_and_change_it() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.get_mut(1).unwrap() = 4;
    *list.get_mut(2).unwrap() = 5;
    for (i, e) in (0..list.len()).zip([3, 4, 5, 1, 2, 3]) {
      assert_eq!(*list.get(i).unwrap(), e);
    }
  }

  #[test_case]
  fn iter() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.iter_mut().nth(1).unwrap() = 4;
    *list.iter_mut().nth(2).unwrap() = 5;
    let vec = list.iter().cloned().collect::<Vec<_>>();
    assert_eq!(vec, [3, 4, 5, 1, 2, 3]);
  }

  #[test_case]
  fn rev_iter() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.iter_mut().rev().nth(1).unwrap() = 4;
    *list.iter_mut().rev().nth(2).unwrap() = 5;
    let vec = list.iter().cloned().collect::<Vec<_>>();
    assert_eq!(vec, [3, 2, 1, 5, 4, 3]);
  }

  #[test_case]
  fn eq_and_clone() {
    let list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    let mut cloned = list.clone();
    assert_eq!(list, cloned);
    cloned.pop_front();
    assert_ne!(list, cloned);
  }

  #[test_case]
  fn format() {
    let list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    assert_eq!(format!("{}", list), "[3, 2, 1, 1, 2, 3]");
  }

  #[test_case]
  fn clear_and_drop() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    assert_eq!(format!("{}", list), "[3, 2, 1, 1, 2, 3]");
    assert_eq!(6, list.len());
    list.clear();
    assert_eq!(format!("{}", list), "[]");
    assert_eq!(0, list.len());
    list.push_back(1);
    assert_eq!(format!("{}", list), "[1]");
    assert_eq!(1, list.len());
    drop(list);
  }
}
