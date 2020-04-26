#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, external_doc))]
#![cfg_attr(docsrs, doc(include = "../README.md"))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod comparator;
mod sorted_slice;
#[cfg(feature = "alloc")]
mod sorted_vec;
mod utils;
mod weak_borrow;

pub use crate::comparator::*;
pub use crate::sorted_slice::*;
#[cfg(feature = "alloc")]
pub use crate::sorted_vec::*;
pub use crate::weak_borrow::*;

pub mod prelude {
    pub use crate::sorted_slice::SortedSlice;
    #[cfg(feature = "alloc")]
    pub use crate::sorted_vec::SortedVec;
}
