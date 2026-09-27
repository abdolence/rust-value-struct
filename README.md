[![Cargo](https://img.shields.io/crates/v/rvstruct.svg)](https://crates.io/crates/rvstruct)
[![tests & formatting](https://github.com/abdolence/rust-value-struct/actions/workflows/tests.yml/badge.svg)](https://github.com/abdolence/rust-value-struct/actions/workflows/tests.yml)

# Value Structs derive macros for Rust to support the newtype pattern

## Motivation
A very simple derive macros to support strong type system and [the newtype pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
Newtypes are a zero-cost abstraction: they introduce a new, distinct name for an existing type, with no runtime overhead when converting between the two types.
This is a similar approach to Haskell's [newtype keyword](https://wiki.haskell.org/Newtype).

For example:
```rust
use rvstruct::ValueStruct;

#[derive(ValueStruct)]
struct UserId(String);

let uid: UserId = "my-uid".into();
```

## Macros overview

`ValueStruct` generates for you:
 - `new()` const function to create a new instance of your struct without using `.into()`;
 - `ValueStruct::value()` function implementation to access your field directly without using .0;
 - `ValueStruct::into_value()` function to convert it back to the raw type without cloning;
 - `From<T>` and `From<&T>` instances to help you to create your structs, where `T` is the field type. `From<&T>` clones the value, so `T` needs `Clone` or its own `clone()` method.

There are different behaviour for different field types:
 - for `String` it generates additionally `From<&str>`, `FromStr`, `AsRef<str>` and `Display`;
 - for integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`) it generates additionally `Display`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rvstruct = "0.3"
```

```rust
// Import it
use rvstruct::ValueStruct;

// And use it on your structs
#[derive(ValueStruct)]
struct UserId(String);

let user_id = UserId::new("my-uid".to_string());

// Reading the raw value
let user_id_str: &String = user_id.value();
assert_eq!(user_id_str, "my-uid");

// Converting it back to the raw type
let user_id_string: String = user_id.into_value();
assert_eq!(user_id_string, "my-uid");
```

The minimum supported Rust version is 1.71.

## License
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
