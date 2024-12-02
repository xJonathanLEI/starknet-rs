use starknet_types_core::{
    felt::Felt,
    hash::{Pedersen, StarkHash},
};

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
///
/// ### Parameters
///
/// - `x`: The x coordinate.
/// - `y`: The y coordinate.
pub fn pedersen_hash(x: &Felt, y: &Felt) -> Felt {
    Pedersen::hash(x, y)
}
