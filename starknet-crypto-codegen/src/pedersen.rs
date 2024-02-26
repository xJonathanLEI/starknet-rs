// Code ported from the build.rs script here:
//   https://github.com/eqlabs/pathfinder/blob/7f9a6bb0264943f93a633f61fc4e0bc9237f68a0/crates/stark_hash/build.rs

use std::fmt::Write;

use proc_macro::TokenStream;
use starknet_curve::{
    curve_params::{PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, PEDERSEN_P3},
    AffinePoint,
};
use syn::{parse_macro_input, LitInt};

pub fn lookup_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let bits: u32 = input.base10_parse().expect("invalid bits");

    let mut output = String::new();
    writeln!(output, "pub const CURVE_CONSTS_BITS: usize = {bits};").unwrap();

    push_points(&mut output, "P0", PEDERSEN_P0, 248, bits).expect("push_points failed");
    push_points(&mut output, "P1", PEDERSEN_P1, 4, bits).expect("push_points failed");
    push_points(&mut output, "P2", PEDERSEN_P2, 248, bits).expect("push_points failed");
    push_points(&mut output, "P3", PEDERSEN_P3, 4, bits).expect("push_points failed");

    output.parse().unwrap()
}

fn push_points(
    buf: &mut String,
    name: &str,
    base: AffinePoint,
    max_bits: u32,
    bits: u32,
) -> std::fmt::Result {
    let full_chunks = max_bits / bits;
    let leftover_bits = max_bits % bits;
    let table_size_full = (1 << bits) - 1;
    let table_size_leftover = (1 << leftover_bits) - 1;
    let len = full_chunks * table_size_full + table_size_leftover;

    writeln!(
        buf,
        "pub const CURVE_CONSTS_{name}: [::starknet_curve::AffinePoint; {len}] = ["
    )?;

    let mut bits_left = max_bits;
    let mut outer_point = base;
    while bits_left > 0 {
        let eat_bits = std::cmp::min(bits_left, bits);
        let table_size = (1 << eat_bits) - 1;

        // Loop through each possible bit combination except zero
        let mut inner_point = outer_point;
        for _ in 1..(table_size + 1) {
            push_point(buf, &inner_point)?;
            inner_point += &outer_point;
        }

        // Shift outer point #bits times
        bits_left -= eat_bits;
        for _i in 0..bits {
            outer_point.double_assign();
        }
    }

    writeln!(buf, "];")?;
    Ok(())
}

fn push_point(buf: &mut String, p: &AffinePoint) -> std::fmt::Result {
    let x = p.x.into_mont();
    let y = p.y.into_mont();
    writeln!(buf, "::starknet_curve::AffinePoint {{")?;
    writeln!(buf, "x: ::starknet_ff::FieldElement::from_mont([")?;
    writeln!(buf, "{},", x[0])?;
    writeln!(buf, "{},", x[1])?;
    writeln!(buf, "{},", x[2])?;
    writeln!(buf, "{},", x[3])?;
    writeln!(buf, "]),")?;
    writeln!(buf, "y: ::starknet_ff::FieldElement::from_mont([")?;
    writeln!(buf, "{},", y[0])?;
    writeln!(buf, "{},", y[1])?;
    writeln!(buf, "{},", y[2])?;
    writeln!(buf, "{},", y[3])?;
    writeln!(buf, "]),")?;
    writeln!(buf, "infinity: false,")?;
    writeln!(buf, "}},")?;
    Ok(())
}
