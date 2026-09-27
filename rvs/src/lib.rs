//! Value Structs derive macros for Rust to support the newtype pattern
//!
//! A very simple derive macros to support strong type system and
//! the [newtype pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
//!
//! For example:
//! ```
//! use rvstruct::ValueStruct;
//!
//! #[derive(ValueStruct)]
//! struct UserId(String);
//!
//! let uid: UserId = "my-uid".into();
//! assert_eq!(uid.value(), "my-uid");
//! ```
//!
//! `ValueStruct` generates for any field type:
//!  - `new()` const function to create your struct without `.into()`;
//!  - [`ValueStruct::value()`] to access your field without using `.0`;
//!  - [`ValueStruct::into_value()`] to convert it back to the field type without cloning;
//!  - `From<T>` and `From<&T>`, where `T` is the field type.
//!
//! Depending on the field type, it also generates:
//!  - for `String` or `std::string::String`: `From<&str>`, `FromStr`, `AsRef<str>` and `Display`;
//!  - for the integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`,
//!    `u32`, `u64`, `u128`, `usize`): `Display`.
//!
//! The field type is recognised by how it is written, so a type alias for `String`
//! gets only the impls for any field type.

pub use rvs_derive::*;

/// Access to the single field of a value struct.
///
/// Implemented by `#[derive(ValueStruct)]`.
pub trait ValueStruct {
    /// The type of the wrapped field.
    type ValueType;

    /// Returns a reference to the wrapped field.
    fn value(&self) -> &Self::ValueType;

    /// Consumes the struct and returns the wrapped field.
    fn into_value(self) -> Self::ValueType;
}
