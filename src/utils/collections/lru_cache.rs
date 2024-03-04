use crate::println;
use alloc::boxed::Box;
use core::{borrow::Borrow, hash::Hash, num::NonZeroUsize, ptr::NonNull};
use hashbrown::HashMap;

pub const DEFAULT_CAPACITY: usize = 8;

struct LruCacheNode<K, V> {
  key: K,
  value: V,
  prev: Option<NonNull<LruCacheNode<K, V>>>,
  next: Option<NonNull<LruCacheNode<K, V>>>,
}

impl<K, V> LruCacheNode<K, V> {
  fn new(key: K, value: V) -> Self {
    Self {
      key,
      value,
      prev: None,
      next: None,
    }
  }
}

/// A wrapper of `NonNull<Node<K, V>>` which forwards:
///
/// - [`Hash`]
/// - [`Borrow`]
/// - [`PartialEq`]
/// - [`Eq`]
///
/// to the `key` of the node (alias: `node->key`).
pub(crate) struct KeyRef<K, V>(NonNull<LruCacheNode<K, V>>);

impl<K: Eq, V> Eq for KeyRef<K, V> {}

impl<K: PartialEq, V> PartialEq for KeyRef<K, V> {
  fn eq(&self, other: &Self) -> bool {
    unsafe { self.0.as_ref().key == other.0.as_ref().key }
  }
}

impl<K: Hash + Eq, V> Hash for KeyRef<K, V> {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    unsafe { self.0.as_ref().key.hash(state) }
  }
}

impl<K: Hash + Eq, V> Borrow<K> for KeyRef<K, V> {
  fn borrow(&self) -> &K {
    unsafe { &self.0.as_ref().key }
  }
}

pub struct LruCache<K, V> {
  head: Option<NonNull<LruCacheNode<K, V>>>,
  tail: Option<NonNull<LruCacheNode<K, V>>>,
  map: HashMap<KeyRef<K, V>, NonNull<LruCacheNode<K, V>>>,
  capacity: NonZeroUsize,
}

impl<K, V> Default for LruCache<K, V> {
  fn default() -> Self {
    Self {
      head: None,
      tail: None,
      map: HashMap::default(),
      capacity: NonZeroUsize::new(DEFAULT_CAPACITY).unwrap(),
    }
  }
}

impl<K, V> Drop for LruCache<K, V> {
  fn drop(&mut self) {
    while let Some(node) = self.head.take() {
      unsafe {
        self.head = node.as_ref().next;
        drop(Box::from_raw(node.as_ptr()));
      }
    }
  }
}

impl<K: PartialEq, V> LruCache<K, V> {
  /// to detach given `node` from `linked_list`
  fn detach(&mut self, mut node: NonNull<LruCacheNode<K, V>>) {
    let prev = unsafe { node.as_mut().prev };
    let next = unsafe { node.as_mut().next };

    match prev {
      Some(mut prev) => unsafe { prev.as_mut().next = next },
      None => self.head = next,
    }
    match next {
      Some(mut next) => unsafe { next.as_mut().prev = prev },
      None => self.tail = prev,
    }

    unsafe {
      node.as_mut().prev = None;
      node.as_mut().next = None;
    }
  }

  /// to detach given `node` into `linked_list`'s head
  fn attach_head(&mut self, mut node: NonNull<LruCacheNode<K, V>>) {
    match self.head {
      Some(mut head) => {
        unsafe {
          node.as_mut().prev = None;
          node.as_mut().next = Some(head);
          head.as_mut().prev = Some(node);
        }
        self.head = Some(node);
      }
      None => {
        unsafe {
          node.as_mut().prev = None;
          node.as_mut().next = None;
        }
        self.head = Some(node);
        self.tail = Some(node);
      }
    }
  }

  /// to detach given `node` into `linked_list`'s tail
  #[allow(dead_code)]
  fn attach_tail(&mut self, mut node: NonNull<LruCacheNode<K, V>>) {
    match self.tail {
      Some(mut tail) => {
        unsafe {
          node.as_mut().prev = Some(tail);
          node.as_mut().next = None;
          tail.as_mut().next = Some(node);
        }
        self.tail = Some(node);
      }
      None => {
        unsafe {
          node.as_mut().prev = None;
          node.as_mut().next = None;
        }
        self.head = Some(node);
        self.tail = Some(node);
      }
    }
  }
}

impl<K: Hash + Eq, V> LruCache<K, V> {
  pub fn put(&mut self, key: K, value: V) -> Option<V> {
    // new node
    let node = Box::leak(Box::new(LruCacheNode::new(key, value))).into();

    // remove old key (if it exists)
    let old_node = self.map.remove(&KeyRef(node)).map(|node| {
      // it exists, then detach
      self.detach(node);
      node
    });

    // if it's over capacity: remove tail node
    if self.map.len() >= self.capacity.into() {
      let tail = self.tail.unwrap();
      self.detach(tail);
      self.map.remove(&KeyRef(tail));
    }

    // attach new node to head & record it
    self.attach_head(node);
    self.map.insert(KeyRef(node), node);

    // if key exists, return old value
    old_node.map(|node| unsafe { Box::from_raw(node.as_ptr()).value })
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    // query
    if let Some(node) = self.map.get(key) {
      let node = *node;
      // this is the new `latest used` node
      self.detach(node);
      self.attach_head(node);
      // return the value
      Some(unsafe { &node.as_ref().value })
    } else {
      None
    }
  }

  pub fn get_unwrapped(&mut self, key: &K) -> &V {
    self.get(key).unwrap()
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    // query
    if let Some(node) = self.map.get(key) {
      let mut node = *node;
      // this is the new `latest used` node
      self.detach(node);
      self.attach_head(node);
      // return the value
      Some(unsafe { &mut node.as_mut().value })
    } else {
      None
    }
  }

  pub fn get_mut_unwrapped(&mut self, key: &K) -> &mut V {
    self.get_mut(key).unwrap()
  }

  pub fn remove(&mut self, key: &K) -> Option<V> {
    // remove from map
    if let Some(node) = self.map.remove(key) {
      // remove from linked list
      self.detach(node);
      // return the value
      Some(unsafe { Box::from_raw(node.as_ptr()).value })
    } else {
      None
    }
  }

  pub fn contains_key(&self, key: &K) -> bool {
    self.map.contains_key(key)
  }
}

impl<K, V> LruCache<K, V> {
  pub fn new(capacity: impl TryInto<NonZeroUsize>) -> Self {
    Self {
      head: None,
      tail: None,
      map: HashMap::new(),
      capacity: capacity
        .try_into()
        .unwrap_or_else(|_| {
          println!("Input `capacity` cannot converts to a [`NonZeroUsize`], use `DEFAULT_CAPACITY = {}` instead.", DEFAULT_CAPACITY);
          NonZeroUsize::new(DEFAULT_CAPACITY).unwrap()
        }),
    }
  }

  pub fn len(&self) -> usize {
    self.map.len()
  }

  pub fn capacity(&self) -> usize {
    self.capacity.into()
  }

  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  pub fn clear(&mut self) {
    self.map.clear();

    while let Some(node) = self.head.take() {
      unsafe {
        self.head = node.as_ref().next;
        drop(Box::from_raw(node.as_ptr()));
      }
    }

    self.head = None;
    self.tail = None;
  }
}
