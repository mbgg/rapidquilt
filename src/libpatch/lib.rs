// Licensed under the MIT license. See LICENSE.md

#![cfg_attr(feature = "bencher", feature(test))]
#[cfg(feature = "bencher")]
extern crate test;

pub mod interned_file;
pub mod line_interner;
pub mod patch;
mod util;

#[cfg(test)]
mod tests;