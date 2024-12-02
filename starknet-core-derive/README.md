# Procedural macros for `starknet-core`

This crate provides procedural macros for deriving the `Encode` and `Decode` traits from `starknet-core`. This allows defining a type like:

```rust
#[derive(Debug, PartialEq, Eq, Decode, Encode)]
struct CairoType {
    a: Felt,
    b: U256,
    c: bool,
}
```

and using the `::encode()` and `::decode()` methods, without manually implementing the corresponding traits.
