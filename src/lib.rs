//! The [`Sorted`] type is a wrapper type for array-like types.

mod macros;
mod sorted;

#[cfg(test)]
mod test;

#[doc(inline)]
pub use self::sorted::*;
