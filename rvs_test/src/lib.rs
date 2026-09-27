#[cfg(test)]
mod tests {

    use rvstruct::ValueStruct;

    #[derive(Debug, ValueStruct, Clone)]
    struct SimpleStrValueStruct(String);

    #[derive(Debug, ValueStruct, Clone)]
    struct StdStrValueStruct(std::string::String);

    #[derive(Debug, ValueStruct, Clone)]
    struct SimpleIntValueStruct(u8);

    #[derive(ValueStruct)]
    struct UserId(String);

    /// Generated code must not resolve `Result`, `Ok` or `String` against the
    /// names in the user's module, where a crate-local `Result<T>` alias is common.
    mod result_alias {
        use rvstruct::ValueStruct;

        #[derive(Debug)]
        pub struct AppError;

        pub type Result<T> = std::result::Result<T, AppError>;

        #[derive(Debug, ValueStruct)]
        pub struct Name(String);

        pub fn parse_name(s: &str) -> Result<Name> {
            s.parse().map_err(|_| AppError)
        }
    }

    #[test]
    fn create_str_value_struct() {
        let s1: SimpleStrValueStruct = String::from("Hey").into();
        assert_eq!(s1.value(), "Hey");

        let s12: SimpleStrValueStruct = "Hey".into();
        assert_eq!(s12.value(), "Hey");

        let s13 = SimpleStrValueStruct::from(s12.value());
        assert_eq!(s13.value(), "Hey");

        let s14 = SimpleStrValueStruct::new("Hey".to_string());
        assert_eq!(s14.value(), "Hey");
    }

    #[test]
    fn create_std_str_value_struct() {
        let s1: StdStrValueStruct = std::string::String::from("Hey").into();
        assert_eq!(s1.value(), "Hey");
    }

    #[test]
    fn create_int_value_struct() {
        let i1: SimpleIntValueStruct = 1u8.into();
        assert_eq!(*i1.value(), 1u8);
    }

    #[test]
    fn create_example_struct() {
        let uid: UserId = "my-uid".into();
        assert_eq!(uid.value(), "my-uid");
    }

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        let uid: UserId = "my-uid".parse()?;
        assert_eq!(uid.value(), "my-uid");
        Ok(())
    }

    #[test]
    fn test_func_as_param() {
        fn test_func(id: &UserId) -> &UserId {
            id
        }

        fn test_func_str(id: &str) -> &str {
            id
        }
        let uid = "my-uid".into();
        let uid_fres: &UserId = test_func(&uid);
        let str_fres = test_func_str(uid.as_ref());
        assert_eq!(uid_fres.value(), "my-uid");
        assert_eq!(str_fres, "my-uid");
    }

    #[test]
    fn test_displayable() {
        let test: SimpleStrValueStruct = "test".into();
        assert_eq!(format!("{}", test), "test");
    }

    #[test]
    fn into_value_returns_the_field() {
        let s: SimpleStrValueStruct = "Hey".into();
        let raw: String = s.into_value();
        assert_eq!(raw, "Hey");

        let i = SimpleIntValueStruct::new(7);
        assert_eq!(i.into_value(), 7u8);
    }

    #[test]
    fn from_reference_clones_the_value() {
        let raw = String::from("Hey");
        let s = SimpleStrValueStruct::from(&raw);
        assert_eq!(s.value(), &raw);

        let i = SimpleIntValueStruct::from(&42u8);
        assert_eq!(*i.value(), 42);
    }

    #[test]
    fn display_writes_the_raw_value() {
        let s: SimpleStrValueStruct = "a \"quoted\" value".into();
        assert_eq!(s.to_string(), "a \"quoted\" value");
        assert_eq!(format!("[{:>5}]", StdStrValueStruct::from("ab")), "[   ab]");

        let i: SimpleIntValueStruct = 42u8.into();
        assert_eq!(i.to_string(), "42");
        assert_eq!(format!("{:03}", i), "042");
    }

    #[test]
    fn as_ref_str_borrows_the_value() {
        let uid: UserId = "my-uid".into();
        let s: &str = uid.as_ref();
        assert_eq!(s, "my-uid");
    }

    #[test]
    fn derives_with_result_alias_in_scope() {
        let name = result_alias::parse_name("Alice").unwrap();
        assert_eq!(name.value(), "Alice");
        assert_eq!(name.to_string(), "Alice");
    }
}
