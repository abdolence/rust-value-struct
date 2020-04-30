[![Cargo](https://img.shields.io/crates/v/rust-value-strucct.svg)](
https://crates.io/crates/rust-value-struct)

# Value Structs ("classes") derive macros for Rust

## Motivation
A very simple derive macros to support strong type system and avoid bare types (like String) 
for domain types using Rust structs with exactly one unnamed field as a immutable value type.

This is similar approach to Haskell's `newtype` (https://wiki.haskell.org/Newtype) or Scala's `AnyVal`. 

e.g. to declare something like this:
```rust
#[derive(ValueStruct)]
struct UserId(String);

let uid : UserId = "my-uid".into();
```

## Macros overview

`ValueStruct` generates for you:
 - `std::convert::From<>` instances automatically to help you to create your structs.
 - an inline `value()` function to access your field directly without using .0.

There are different behaviour for different field types:
- For `std::string::String` it generates `From<String>`, `From<&String>`, `From<&str>`
- For scalar types `value()` isn't a reference, for others it is.
 
## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rvs_derive = "0.1"
``` 

## Licence
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
