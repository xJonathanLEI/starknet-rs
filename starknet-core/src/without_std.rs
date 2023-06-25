#[macro_use]
extern crate alloc;

pub mod without_std {
    pub use alloc::{borrow, string, sync, vec, boxed, collections};
    pub use core::{fmt, mem};
    pub use thiserror_no_std::Error;
    
}