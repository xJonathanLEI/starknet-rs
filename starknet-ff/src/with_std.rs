// Inspired by Substrate sp-std crate
// see https://github.com/paritytech/substrate/blob/master/primitives/std/with_std.rs

pub use core::time;
pub use std::alloc;
pub use std::any;
pub use std::borrow;
pub use std::boxed;
pub use std::cell;
pub use std::clone;
pub use std::cmp;
pub use std::convert;
pub use std::default;
pub use std::fmt;
pub use std::hash;
pub use std::iter;
pub use std::marker;
pub use std::mem;
pub use std::num;
pub use std::ops;
pub use std::ptr;
pub use std::rc;
pub use std::result;
pub use std::slice;
pub use std::str;
pub use std::string;
pub use std::sync;
pub use std::vec;

pub mod collections {
    pub use std::collections::{HashMap, HashSet};
}

pub mod thread {
    pub use std::thread::panicking;
}
