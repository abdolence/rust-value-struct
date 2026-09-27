//! A field spelled `String` gets the `String` impls, and those impls have to
//! work with whatever `String` names in the user's module.

mod small {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub struct SmallString(std::string::String);

    impl SmallString {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl From<&str> for SmallString {
        fn from(value: &str) -> Self {
            SmallString(value.to_owned())
        }
    }

    impl fmt::Display for SmallString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "small:{}", self.0)
        }
    }
}

mod imported_as_string {
    use crate::small::SmallString as String;
    use rvstruct::ValueStruct;

    #[derive(Debug, ValueStruct)]
    pub struct Tag(String);
}

mod aliased_as_string {
    use rvstruct::ValueStruct;

    type String = crate::small::SmallString;

    #[derive(Debug, ValueStruct)]
    pub struct Tag(String);
}

#[test]
fn string_impls_use_the_string_in_scope() {
    use rvstruct::ValueStruct;

    let tag: imported_as_string::Tag = "a".into();
    assert_eq!(tag.value().as_str(), "a");
    let parsed: imported_as_string::Tag = "b".parse().unwrap();
    assert_eq!(parsed.as_ref(), "b");
    assert_eq!(parsed.to_string(), "small:b");

    let tag: aliased_as_string::Tag = "c".into();
    assert_eq!(tag.value().as_str(), "c");
    let parsed: aliased_as_string::Tag = "d".parse().unwrap();
    assert_eq!(parsed.as_ref(), "d");
    assert_eq!(parsed.to_string(), "small:d");
}
