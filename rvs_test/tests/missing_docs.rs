//! The derive must compile in crates that deny `missing_docs`.
#![deny(missing_docs)]

use rvstruct::ValueStruct;

/// A value struct over a `String` field.
#[derive(ValueStruct)]
pub struct Name(pub String);

/// A value struct over a non-`String` field.
#[derive(ValueStruct)]
pub struct CipherText(pub Vec<u8>);

#[test]
fn derived_types_compile_under_deny_missing_docs() {
    assert_eq!(Name::new("a".to_string()).value(), "a");
    assert_eq!(CipherText::new(vec![1]).value(), &vec![1]);
}
