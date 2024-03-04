#![allow(unused_imports)]

pub mod prelude;

pub use crate::utils::collections::{linked_list, lru_cache, trie, vec};

#[cfg(test)]
mod test_collection_alias {
  use super::*;

  #[test_case]
  fn build_vec() {
    let a = vec::Vec::from_iter(0..3);
    assert_eq!(a[..], [0, 1, 2])
  }

  #[test_case]
  fn build_linked_list() {
    let a = linked_list::LinkedList::from_iter(0..3);
    assert_eq!(a.iter().cloned().collect::<vec::Vec<_>>()[..], [0, 1, 2])
  }

  #[test_case]
  fn lru_cache() {
    let mut a = lru_cache::LruCache::new(3);
    assert_eq!(a.capacity(), 3);
    for (k, v) in ['a', 'b', 'c'].into_iter().enumerate() {
      a.put(k, v);
    }
  }

  #[test_case]
  fn trie() {
    let mut trie = trie::Trie::new();
    trie.insert("apple".chars());
    assert!(trie.search("apple".chars()));
    assert!(!trie.search("app".chars()));
    assert!(trie.starts_with("apple".chars()));
    assert!(trie.starts_with("app".chars()));
  }
}
