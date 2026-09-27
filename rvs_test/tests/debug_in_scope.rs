use rvstruct::ValueStruct;
#[allow(unused_imports)]
use std::fmt::Debug;

#[derive(Debug, ValueStruct)]
struct Name(String);

#[derive(Debug, ValueStruct)]
struct Count(u32);

#[test]
fn display_is_unambiguous_with_debug_in_scope() {
    assert_eq!(Name::from("Alice").to_string(), "Alice");
    assert_eq!(Count::from(3).to_string(), "3");
}
