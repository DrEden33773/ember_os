use super::{align_up, Locked};
use core::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

struct ListNode {
  /// Size of current list_node
  size: usize,
  /// Next list_node (lifetime = 'static, for the linked_list exists for all time)
  next: Option<&'static mut ListNode>,
}

impl ListNode {
  /// Create a list_node
  const fn new(size: usize) -> Self {
    ListNode { size, next: None }
  }
  /// Get start address of current list_node
  fn start_addr(&self) -> usize {
    self as *const Self as usize
  }
  /// Get end address of current list_node
  fn end_addr(&self) -> usize {
    self.start_addr() + self.size
  }
}

pub struct LinkedListAllocator {
  head: ListNode,
}

impl LinkedListAllocator {
  /// Creates an empty LinkedListAllocator.
  pub const fn new() -> Self {
    Self {
      head: ListNode::new(0),
    }
  }

  /// Initialize the allocator with the given heap bounds.
  ///
  /// # Safety
  ///
  /// This function is `unsafe` because the caller must ensure that the given
  /// heap bounds are `valid` and that the heap is `unused`.
  ///
  /// This method must be called `only once`.
  pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
    self.add_free_region(heap_start, heap_size);
  }

  /// Adds the given memory region to the front of the list.
  unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
    assert_eq!(
      align_up(addr, mem::align_of::<ListNode>()),
      addr,
      "unexpected align of free region occurs!\n"
    );
    assert!(
      size >= mem::size_of::<ListNode>(),
      "heap_size < sizeof(ListNode) => NO free region!\n"
    );

    // create a new list node and append it at the start of the list
    let mut node = ListNode::new(size);
    node.next = self.head.next.take();
    let node_ptr = addr as *mut ListNode;
    node_ptr.write(node);
    self.head.next = Some(&mut *node_ptr);
  }
}

impl Default for LinkedListAllocator {
  fn default() -> Self {
    Self::new()
  }
}

impl LinkedListAllocator {
  /// Looks for a free region with the given size and alignment.
  ///
  /// Then removes it from the list.
  ///
  /// Returns a tuple of the `list node` and the `start address` of the allocation.
  fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
    // reference to current list node, updated for each iteration
    let mut current = &mut self.head;

    // look for a large enough memory region in linked list
    while let Some(ref mut region) = current.next {
      if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
        // region suitable for allocation -> remove node from list
        let next = region.next.take();
        let ret = Some((current.next.take().unwrap(), alloc_start));
        current.next = next;
        return ret;
      }
      // region not suitable -> continue with next region
      current = current.next.as_mut().unwrap();
    }

    // no suitable region found
    None
  }

  /// Try to use the given region for an allocation
  /// with given size and alignment.
  ///
  /// Returns the allocation start address on success.
  fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
    let alloc_start = align_up(region.start_addr(), align);
    let alloc_end = alloc_start.checked_add(size).ok_or(())?;

    // region too small
    if alloc_end > region.end_addr() {
      return Err(());
    }

    let excess_size = region.end_addr() - alloc_end;
    // rest of region too small to hold a ListNode (required because the
    // allocation splits the region in a used and a free part)
    if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
      // `excess_size == 0` is legal!
      return Err(());
    }

    // region suitable for allocation
    Ok(alloc_start)
  }
}

impl LinkedListAllocator {
  /// Adjust the given layout so that the resulting allocated memory
  /// region is also capable of storing a `ListNode`.
  ///
  /// Returns the adjusted size and alignment as a (size, align) tuple.
  fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
      .align_to(mem::align_of::<ListNode>())
      .expect("adjusting alignment failed!\n")
      .pad_to_align();
    let size = layout.size().max(mem::size_of::<ListNode>());
    (size, layout.align())
  }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    // perform layout adjustments
    let (size, align) = LinkedListAllocator::size_align(layout);
    let mut allocator = self.lock();

    // try to find available region
    if let Some((region, alloc_start)) = allocator.find_region(size, align) {
      let alloc_end = alloc_start.checked_add(size).expect("overflow!\n");
      let excess_size = region.end_addr() - alloc_end;
      // dynamically add a free region to the tail
      if excess_size > 0 {
        allocator.add_free_region(alloc_end, excess_size);
      }
      alloc_start as *mut u8
    } else {
      ptr::null_mut()
    }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    // perform layout adjustments
    let (size, _) = LinkedListAllocator::size_align(layout);

    self.lock().add_free_region(ptr as usize, size);
  }
}
