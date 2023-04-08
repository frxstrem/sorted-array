#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod comparator;
mod sorted_array;
mod sorted_slice;
#[cfg(feature = "alloc")]
mod sorted_vec;
mod utils;
mod weak_borrow;

pub use crate::comparator::*;
pub use crate::sorted_array::*;
pub use crate::sorted_slice::*;
#[cfg(feature = "alloc")]
pub use crate::sorted_vec::*;
pub use crate::weak_borrow::*;

pub mod prelude {
    pub use crate::sorted_array::SortedArray;
    pub use crate::sorted_slice::SortedSlice;
    #[cfg(feature = "alloc")]
    pub use crate::sorted_vec::SortedVec;
}
