use super::*;

pub use hack::into_vec;
pub use hack::VecSlice;

pub(crate) mod hack {
  use super::*;
  use alloc::boxed::Box;

  pub trait VecSlice<T> {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
  }

  impl<T> VecSlice<T> for Vec<T> {
    fn as_slice(&self) -> &[T] {
      unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
      unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) }
    }
  }

  pub fn into_vec<T>(b: Box<[T]>) -> Vec<T> {
    unsafe {
      let len = b.len();
      let data = Box::leak(b) as *mut [T] as *mut T;
      Vec::from_unchecked(data, len)
    }
  }
}

macro_rules! impl_to_slice {
  ($range_type:ty) => {
    impl<T> core::ops::Index<$range_type> for Vec<T> {
      type Output = [T];

      fn index(&self, index: $range_type) -> &Self::Output {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.len) }
          .get(index)
          .unwrap()
      }
    }

    impl<T> core::ops::IndexMut<$range_type> for Vec<T> {
      fn index_mut(&mut self, index: $range_type) -> &mut Self::Output {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) }
          .get_mut(index)
          .unwrap()
      }
    }
  };
}

// &vec[l..r], &mut vec[l..r]
impl_to_slice!(core::ops::Range<usize>);

// &vec[l..=r], &mut vec[l..=r]
impl_to_slice!(core::ops::RangeInclusive<usize>);

// &vec[l..], &mut vec[l..]
impl_to_slice!(core::ops::RangeFrom<usize>);

// &vec[..r], &mut vec[..r]
impl_to_slice!(core::ops::RangeTo<usize>);

// &vec[..=r], &mut vec[..=r]
impl_to_slice!(core::ops::RangeToInclusive<usize>);

// &vec[..], &mut vec[..]
impl_to_slice!(core::ops::RangeFull);
