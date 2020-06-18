//! Value Structs derive macros for Rust to support the newtype pattern
//!
//! A very simple derive macros to support strong type system and
//! the new type pattern (https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
//!
//! For example:
//! ```
//! use rvs::ValueStruct;
//!
//! #[derive(ValueStruct)]
//! struct UserId(String);
//!
//! let uid : UserId = "my-uid".into();
//! ```
//!
//! `ValueStruct` generates for you:
//!  - `std::convert::From<>` instances automatically to help you to create your structs.
//!  - `ValueStruct::value()` function to access your field directly without using .0.
//!
//! There are different behaviour for different field types:
//! - for `std::string::String` it generates additional instance for `From<&str>`
//!

use rvs_derive;
pub use rvs_derive::*;

pub trait ValueStruct {
    type ValueType;

    fn value(&self) -> &Self::ValueType;
}
