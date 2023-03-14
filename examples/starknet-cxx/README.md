# Example usage of starknet-rs from C++

This is a quick demo on exposing `starknet-crypto` to C++ with the [cxx](https://github.com/dtolnay/cxx) bridge and [corrosion](https://github.com/corrosion-rs/corrosion).

## **WARNING**

As noted in the [`starknet-crypto` page](../../starknet-crypto/), you're advised to use high-level constructs exposed through `starknet-core` instead if you're not familiar with cryptographic primitives. `starknet-crypto` is chosen here considering that the C++ audience might prefer low-level libraries, but it's possible to wrap `starknet-core` as well.

## Note

This wrapper crate expose functions that operate on strings, which is bad and probably hurts performance. It's possible to make the C++ side create `FieldElement` instances and operate on those instead, which is much more idiomatic. That said, this demo wrapper crate seems to already offer decent performance.

Moreover, this crate does not implement error handling and always just panics on error, which is likely not what you want in production.

However, the goal of this crate is just to demonstrate using the library from C++, _NOT_ to create idiomatic bindings, which is way too much work to maintain as an example, and should be a project of its own.

## Running the example

With necessary toolings installed:

```console
$ mkdir build && cd build
$ cmake -DCMAKE_BUILD_TYPE=Release ..
$ make
```

It everything goes well, you should now have a `main` executable:

```console
$ ./main
pedersen_hash():
  0x030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662
ecdsa_sign():
  0x0411494b501a98abd8262b0da1351e17899a0c4ef23dd2f96fec5ba847310b200405c3191ab3883ef2b763af35bc5f5d15b3b4e99461d70e84c654a351a7c81b
```
