use alloc::boxed::Box;
use core::hash::Hash;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub(crate) struct Trie<T: Hash + Eq> {
  pub(crate) children: HashMap<T, Box<Trie<T>>>,
  pub(crate) is_end: bool,
}

impl<T: Hash + Eq> Trie<T> {
  pub fn new() -> Self {
    Self {
      children: HashMap::new(),
      is_end: false,
    }
  }

  pub fn insert(&mut self, seq: impl Iterator<Item = T>) {
    let mut node = self;
    for c in seq {
      node = node.children.entry(c).or_insert_with(|| Trie::new().into());
    }
    node.is_end = true;
  }

  pub fn search(&self, seq: impl Iterator<Item = T>) -> bool {
    let mut node = self;
    for c in seq {
      if let Some(next) = node.children.get(&c) {
        node = next;
      } else {
        return false;
      }
    }
    node.is_end
  }

  pub fn starts_with(&self, seq: impl Iterator<Item = T>) -> bool {
    let mut node = self;
    for c in seq {
      if let Some(next) = node.children.get(&c) {
        node = next;
      } else {
        return false;
      }
    }
    true
  }
}
