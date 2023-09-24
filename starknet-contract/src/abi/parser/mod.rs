//! This crates is about parsing Cairo types from an ABI.
//! Later, this will also be able to parse Cairo type from Cairo code.
//!
//! The important consideration are the generic type. Indeed, in the ABI
//! there is no information about a type genericity and how exactly
//! the members/variants are following the generic type as everything is
//! flattened.
//!
//! `abi_types` is the low level parsing of the types. It supports
//! nested types.
//!
//! `CairoStruct`, `CairoEnum` and `CairoFunction` are higher level
//! types to resolve the genericity and manage members/variants/inputs/outputs
//! for simpler expansion.
mod abi_types;

mod cairo_struct;
pub use cairo_struct::CairoStruct;

mod cairo_enum;
pub use cairo_enum::CairoEnum;

mod cairo_function;
pub use cairo_function::CairoFunction;

mod cairo_event;
pub use cairo_event::CairoEvent;
