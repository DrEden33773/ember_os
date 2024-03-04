pub mod cached_greedy_extreme_path;
pub mod greedy_extreme_path;

pub struct Edge<Node, Value> {
  pub(crate) dst: Node,
  pub(crate) cost: Value,
}

impl<Node, Value> Edge<Node, Value> {
  pub fn new(dst: Node, cost: Value) -> Self {
    Self { dst, cost }
  }
}

impl<Node: Copy, Value: Copy> Copy for Edge<Node, Value> {}

impl<Node: Clone, Value: Clone> Clone for Edge<Node, Value> {
  fn clone(&self) -> Self {
    Self {
      dst: self.dst.clone(),
      cost: self.cost.clone(),
    }
  }
}

impl<Node: Default, Value: Default> Default for Edge<Node, Value> {
  fn default() -> Self {
    Self {
      dst: Default::default(),
      cost: Default::default(),
    }
  }
}

#[allow(unused)]
pub type MaxHeapAccumulation<Node, Value> = Accumulation<Node, Value, true>;
#[allow(unused)]
pub type MinHeapAccumulation<Node, Value> = Accumulation<Node, Value, false>;
#[allow(unused)]
pub type OriginalOrderedAccumulation<Node, Value> = Accumulation<Node, Value, true>;
#[allow(unused)]
pub type ReversedOrderedAccumulation<Node, Value> = Accumulation<Node, Value, false>;

pub struct Accumulation<Node, Value, const ADAPT_MAX_HEAP: bool = false> {
  pub(crate) dst: Node,
  pub(crate) cost: Value,
}

impl<Node, Value: Ord, const ADAPT_MAX_HEAP: bool> Ord
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    let order = self.cost.cmp(&other.cost);
    if !ADAPT_MAX_HEAP {
      order.reverse()
    } else {
      order
    }
  }
}

impl<Node, Value: PartialOrd, const ADAPT_MAX_HEAP: bool> PartialOrd
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    let wrapped_order = self.cost.partial_cmp(&other.cost);
    if !ADAPT_MAX_HEAP {
      wrapped_order.map(|res| res.reverse())
    } else {
      wrapped_order
    }
  }
}

impl<Node, Value: Eq, const ADAPT_MAX_HEAP: bool> Eq for Accumulation<Node, Value, ADAPT_MAX_HEAP> {}

impl<Node, Value: PartialEq, const ADAPT_MAX_HEAP: bool> PartialEq
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
  fn eq(&self, other: &Self) -> bool {
    self.cost == other.cost
  }
}

impl<Node: Copy, Value: Copy, const ADAPT_MAX_HEAP: bool> Copy
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
}

impl<Node: Clone, Value: Clone, const ADAPT_MAX_HEAP: bool> Clone
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
  fn clone(&self) -> Self {
    Self {
      dst: self.dst.clone(),
      cost: self.cost.clone(),
    }
  }
}

impl<Node: Default, Value: Default, const ADAPT_MAX_HEAP: bool> Default
  for Accumulation<Node, Value, ADAPT_MAX_HEAP>
{
  fn default() -> Self {
    Self {
      dst: Default::default(),
      cost: Default::default(),
    }
  }
}
