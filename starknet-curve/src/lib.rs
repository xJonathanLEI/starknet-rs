#![no_std]
#![doc = include_str!("../README.md")]

mod ec_point;

pub mod curve_params;
pub use ec_point::{AffinePoint, ProjectivePoint};
