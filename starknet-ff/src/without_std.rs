// Inspired by Substrate sp-std crate
// see https://github.com/paritytech/substrate/blob/master/primitives/std/without_std.rs

#[macro_use]
pub extern crate alloc;

pub use alloc::boxed;
pub use alloc::rc;
pub use alloc::string;
pub use alloc::sync;
pub use alloc::vec;

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::fmt;
pub use core::fmt;
pub use core::hash;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::num;
pub use core::ops;
pub use core::ptr;
pub use core::result;
pub use core::slice;
pub use core::str;
pub use core::time;

pub mod collections {
    pub use hashbrown::{HashMap, HashSet};
}

pub mod borrow {
    pub use alloc::borrow::*;
    pub use core::borrow::*;
}

pub mod thread {
    /// Returns if the current thread is panicking.
    ///
    /// In wasm this always returns `false`, as we abort on any panic.
    pub fn panicking() -> bool {
        false
    }
}
