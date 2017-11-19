#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
extern crate nom;

#[cfg(not(feature = "std"))]
mod std {
#[macro_use]
  pub use core::{fmt, cmp, iter, option, result, ops, slice, str, mem, convert};
  pub use alloc::{boxed, vec, string};
  pub mod prelude {
    pub use core::prelude as v1;
  }
}

pub use self::parser::*;

pub mod parser;
