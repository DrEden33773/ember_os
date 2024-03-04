use super::*;
use alloc::collections::BinaryHeap;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ord;
use core::hash::Hash;
use core::ops::{ControlFlow, Fn};
use hashbrown::HashMap;

pub trait Bounded {
  fn min() -> Self;
  fn max() -> Self;
}

/// To implement [`Bounded`]  for basic types
/// which has `MIN` and `MAX` constants.
#[macro_export]
macro_rules! impl_bounded {
  ($($t:ty),*) => {
    $(
      impl Bounded for $t {
        fn min() -> Self {
          Self::MIN
        }
        fn max() -> Self {
          Self::MAX
        }
      }
    )*
  };
}

impl_bounded!(u8, u16, u32, u64, u128, usize);
impl_bounded!(i8, i16, i32, i64, i128, isize);
impl_bounded!(f32, f64);

pub struct GreedyExtremePathView<'map, Node, Val, BOP, const REVERSED: bool = false>
where
  Node: Hash,
  Val: Ord + Bounded,
  BOP: Fn(Val, Val) -> Val,
{
  cost: HashMap<&'map Node, Val>,
  path: HashMap<&'map Node, &'map Node>,
  heap: BinaryHeap<Accumulation<&'map Node, Val, REVERSED>>,
  src: Option<&'map Node>,
  adj_map: &'map HashMap<Node, Vec<Edge<Node, Val>>>,
  bop: BOP,
  last_accumulation: Option<Accumulation<&'map Node, Val, REVERSED>>,
  self_cost: Val,
}

#[allow(unused)]
pub type GreedyShortestPathView<'map, Node, Val, BOP> =
  GreedyExtremePathView<'map, Node, Val, BOP, false>;

#[allow(unused)]
pub type GreedyLongestPathView<'map, Node, Val, BOP> =
  GreedyExtremePathView<'map, Node, Val, BOP, true>;

impl<'map, Node, Val, BOP, const REVERSED: bool>
  GreedyExtremePathView<'map, Node, Val, BOP, REVERSED>
where
  Node: Hash,
  Val: Ord + Bounded,
  BOP: Fn(Val, Val) -> Val,
{
  pub fn new(adj_map: &'map HashMap<Node, Vec<Edge<Node, Val>>>, bop: BOP, self_cost: Val) -> Self {
    Self {
      cost: HashMap::new(),
      path: HashMap::new(),
      heap: BinaryHeap::new(),
      src: None,
      adj_map,
      bop,
      last_accumulation: None,
      self_cost,
    }
  }
}

impl<'map, Node, Val, BOP, const REVERSED: bool>
  GreedyExtremePathView<'map, Node, Val, BOP, REVERSED>
where
  Node: Hash + Clone + Eq,
  Val: Ord + Clone + Bounded,
  BOP: Fn(Val, Val) -> Val,
{
  fn clear_all_cache(&mut self) {
    self.cost.clear();
    self.path.clear();
    self.heap.clear();
    self.last_accumulation = None;
  }

  fn init(&mut self, node: &'map Node) {
    self.src = Some(node);
    self.cost.insert(node, self.self_cost.clone());
    self.path.insert(node, node);
    self.heap.push(Accumulation {
      dst: node,
      cost: self.self_cost.clone(),
    });
  }

  fn compare_and_swap(&mut self, node: &'map Node) {
    if let Some(old_node) = self.src {
      if old_node != node {
        self.clear_all_cache();
        self.init(node);
      }
    } else {
      self.init(node);
    }
  }

  fn resume_from_last_mutated_query(&mut self, goal: &Node) -> ControlFlow<Option<Val>, ()> {
    if let Some(Accumulation {
      dst: picked,
      cost: src_to_picked,
    }) = self.last_accumulation.clone()
    {
      if picked == goal {
        self.last_accumulation = Some(Accumulation {
          dst: picked,
          cost: src_to_picked.clone(),
        });
        return ControlFlow::Break(Some(src_to_picked));
      }

      if let Some(old_dist) = self.cost.get(picked) {
        if !REVERSED && src_to_picked > *old_dist {
          return ControlFlow::Continue(());
        }
        if REVERSED && src_to_picked < *old_dist {
          return ControlFlow::Continue(());
        }
      }

      let edges = self.adj_map.get(picked);
      if edges.is_none() {
        return ControlFlow::Continue(());
      }

      for Edge {
        dst,
        cost: picked_to_next,
      } in edges.unwrap()
      {
        let src_to_next = (self.bop)(src_to_picked.clone(), picked_to_next.clone());
        let should_update = if !REVERSED {
          src_to_next < *self.cost.get(dst).unwrap_or(&<Val as Bounded>::max())
        } else {
          src_to_next > *self.cost.get(dst).unwrap_or(&<Val as Bounded>::min())
        };
        if should_update {
          self.cost.insert(dst, src_to_next.clone());
          self.path.insert(dst, picked);
          self.heap.push(Accumulation {
            dst,
            cost: src_to_next,
          });
        }
      }
    }

    ControlFlow::Continue(())
  }

  pub fn extreme_cost(&mut self, src: &'map Node, goal: &Node) -> Option<Val> {
    if !self.adj_map.contains_key(src) || !self.adj_map.contains_key(goal) {
      return None;
    }

    self.compare_and_swap(src);

    if let Some(dist) = self.cost.get(goal) {
      return Some(dist.clone());
    }

    match self.resume_from_last_mutated_query(goal) {
      ControlFlow::Break(res) => return res,
      ControlFlow::Continue(_) => {}
    };

    while let Some(Accumulation {
      dst: picked,
      cost: src_to_picked,
    }) = self.heap.pop()
    {
      if picked == goal {
        self.last_accumulation = Some(Accumulation {
          dst: picked,
          cost: src_to_picked.clone(),
        });
        return Some(src_to_picked);
      }

      if let Some(old_dist) = self.cost.get(picked) {
        if !REVERSED && src_to_picked > *old_dist {
          continue;
        }
        if REVERSED && src_to_picked < *old_dist {
          continue;
        }
      }

      for Edge {
        dst,
        cost: picked_to_next,
      } in self.adj_map.get(picked)?
      {
        let src_to_next = (self.bop)(src_to_picked.clone(), picked_to_next.clone());
        let should_update = if !REVERSED {
          src_to_next < *self.cost.get(dst).unwrap_or(&<Val as Bounded>::max())
        } else {
          src_to_next > *self.cost.get(dst).unwrap_or(&<Val as Bounded>::min())
        };
        if should_update {
          self.cost.insert(dst, src_to_next.clone());
          self.path.insert(dst, picked);
          self.heap.push(Accumulation {
            dst,
            cost: src_to_next,
          });
        }
      }
    }

    None
  }

  pub fn extreme_path(&mut self, src: &'map Node, goal: &Node) -> Vec<Node> {
    if !self.adj_map.contains_key(src) || !self.adj_map.contains_key(goal) {
      return vec![];
    }

    // 1. execute `self.extreme_cost()` first
    let cost = self.extreme_cost(src, goal);
    if cost.is_none() {
      return vec![];
    }

    // 2. build result
    let mut result = vec![];
    let mut current = goal;
    while current != src {
      result.push(current.clone());
      current = self.path.get(current).unwrap();
    }
    result.push(src.clone());
    result.reverse();

    // 3. done!
    result
  }
}
