[![Cargo](https://img.shields.io/crates/v/rvs_derive.svg)](https://crates.io/crates/rvs_derive)

# Value Structs derive macros for Rust to support the newtype pattern

## Motivation
A very simple derive macros to support strong type system and [the newtype pattern](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
Newtypes are a zero-cost abstraction: they introduce a new, distinct name for an existing type, with no runtime overhead when converting between the two types. 
This is a similar approach to Haskell's [newtype keyword](https://wiki.haskell.org/Newtype). 

For example:
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

```rust
// Import it
use rvs_derive::ValueStruct;

// And use it on your structs
#[derive(ValueStruct)]
struct UserId(String);

``` 

## Licence
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
