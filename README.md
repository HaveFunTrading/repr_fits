[![Build Status](https://img.shields.io/github/actions/workflow/status/HaveFunTrading/repr_fits/rust.yml)](https://github.com/HaveFunTrading/repr_fits/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/repr_fits.svg)](https://crates.io/crates/repr_fits)
[![Documentation](https://docs.rs/repr_fits/badge.svg)](https://docs.rs/repr_fits/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

`repr_fits` provides a small attribute macro for checking that enum discriminants fit into a fixed number of bits.

This is useful when an enum is stored inside a compact protocol field, packed integer id, file format, or wire representation where Rust's normal `#[repr(u8)]` is too wide. Rust does not have `#[repr(u5)]`, so `repr_fits` lets you keep a normal enum definition while adding compile-time bit-width checks.

## Example

```rust
use repr_fits::repr_fits;

#[repr_fits(bits = 5)]
#[repr(u8)]
enum RegionCode {
    Local = 0,
    Remote = 1,
    Backup = 4,
}
```

The enum remains a normal Rust enum. The macro only appends compile-time assertions for each variant:

```rust
assert!((RegionCode::Local as u128) < (1u128 << 5));
assert!((RegionCode::Remote as u128) < (1u128 << 5));
assert!((RegionCode::Backup as u128) < (1u128 << 5));
```

If a discriminant does not fit, compilation fails with a message naming the offending variant:

```rust,compile_fail
use repr_fits::repr_fits;

#[repr_fits(bits = 2)]
#[repr(u8)]
enum PacketKind {
    Data = 0,
    Ack = 1,
    Control = 4,
}
```

The generated assertion message is shaped like:

```text
PacketKind::Control discriminant does not fit in 2 bits
```

## Why not just use `#[repr(u8)]`?

`#[repr(u8)]` guarantees that discriminants fit in 8 bits. It does not help when the packed representation reserves fewer bits, such as:

```text
5 bits  region code
2 bits  packet kind
25 bits sequence id
```

`repr_fits` documents and enforces that smaller bit budget at compile time.

## Limitations

- Supports enum items only.
- Supports unit variants only.
- The macro checks discriminants by casting variants in a generated `const` assertion block, so the enum must be castable to an integer. In practice, use a primitive representation such as `#[repr(u8)]`.

## License

MIT
