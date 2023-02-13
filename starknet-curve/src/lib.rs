#![doc = include_str!("../README.md")]
#![no_std]

extern crate no_std_compat as std;

mod ec_point;

pub mod curve_params;

pub use ec_point::{AffinePoint, ProjectivePoint};
