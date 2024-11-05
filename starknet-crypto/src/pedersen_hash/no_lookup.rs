// Size-optimized implementation ported from:
//   https://github.com/andrewmilson/sandstorm/blob/9e256c4933aa2d89f794b3ed7c293b32984fe1ce/builtins/src/pedersen/mod.rs#L24-L50

use starknet_curve::curve_params::SHIFT_POINT;
use starknet_types_core::{curve::ProjectivePoint, felt::Felt};

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
///
/// ### Parameters
///
/// - `x`: The x coordinate.
/// - `y`: The y coordinate.
pub fn pedersen_hash(x: &Felt, y: &Felt) -> Felt {
    // Temporarily defining the projective points inline, as `ProjectivePoint::new()` is incorrectly
    // not `const`.
    // TODO: turn these into consts once upstream is fixed.
    let p0_projective: ProjectivePoint = ProjectivePoint::new(
        Felt::from_raw([
            241691544791834578,
            518715844721862878,
            13758484295849329960,
            3602345268353203007,
        ]),
        Felt::from_raw([
            368891789801938570,
            433857700841878496,
            13001553326386915570,
            13441546676070136227,
        ]),
        Felt::ONE,
    );
    let p1_projective: ProjectivePoint = ProjectivePoint::new(
        Felt::from_raw([
            253000153565733272,
            10043949394709899044,
            12382025591154462459,
            16491878934996302286,
        ]),
        Felt::from_raw([
            285630633187035523,
            5191292837124484988,
            2545498000137298346,
            13950428914333633429,
        ]),
        Felt::ONE,
    );
    let p2_projective: ProjectivePoint = ProjectivePoint::new(
        Felt::from_raw([
            338510149841406402,
            12916675983929588442,
            18195981508842736832,
            1203723169299412240,
        ]),
        Felt::from_raw([
            161068411212710156,
            11088962269971685343,
            11743524503750604092,
            12352616181161700245,
        ]),
        Felt::ONE,
    );
    let p3_projective: ProjectivePoint = ProjectivePoint::new(
        Felt::from_raw([
            425493972656615276,
            299781701614706065,
            10664803185694787051,
            1145636535101238356,
        ]),
        Felt::from_raw([
            345457391846365716,
            6033691581221864148,
            4428713245976508844,
            8187986478389849302,
        ]),
        Felt::ONE,
    );

    let processed_x = process_element(x, &p0_projective, &p1_projective);
    let processed_y = process_element(y, &p2_projective, &p3_projective);

    // Unwrapping is safe as this never fails
    (processed_x + processed_y + SHIFT_POINT)
        .to_affine()
        .unwrap()
        .x()
}

#[inline(always)]
fn process_element(x: &Felt, p1: &ProjectivePoint, p2: &ProjectivePoint) -> ProjectivePoint {
    let x = x.to_biguint();
    let shift = 252 - 4;
    let high_part = &x >> shift;
    let low_part = x - (&high_part << shift);
    let x_high = Felt::from(high_part);
    let x_low = Felt::from(low_part);
    p1 * x_low + p2 * x_high
}
