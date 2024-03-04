use alloc::boxed::Box;
use core::hash::Hash;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct Trie<T: Hash + Eq> {
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

#[cfg(test)]
mod generic_trie_test {
  use super::*;

  #[test_case]
  fn string_case() {
    let mut trie = Trie::new();

    trie.insert("apple".chars());
    assert!(trie.search("apple".chars()));
    assert!(!trie.search("app".chars()));
    assert!(trie.starts_with("apple".chars()));
    assert!(trie.starts_with("app".chars()));

    trie.insert("app".chars());
    assert!(trie.search("app".chars()));
    assert!(trie.starts_with("app".chars()));

    trie.insert("appleTree".chars());
    assert!(trie.search("appleTree".chars()));
    assert!(!trie.search("appleT".chars()));
    assert!(trie.starts_with("appleT".chars()));
  }

  #[test_case]
  fn seq_case() {
    let mut trie = Trie::new();

    trie.insert([1, 2, 3].iter());
    assert!(trie.search([1, 2, 3].iter()));
    assert!(!trie.search([1, 2].iter()));
    assert!(trie.starts_with([1, 2, 3].iter()));
    assert!(trie.starts_with([1, 2].iter()));

    trie.insert([1, 2].iter());
    assert!(trie.search([1, 2].iter()));
    assert!(trie.starts_with([1, 2].iter()));

    trie.insert([1, 2, 3, 4, 5].iter());
    assert!(trie.search([1, 2, 3, 4, 5].iter()));
    assert!(!trie.search([1, 2, 3, 4].iter()));
    assert!(trie.starts_with([1, 2, 3, 4].iter()));
  }
}
