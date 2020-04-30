#[cfg(test)]
mod tests {

    use rvs_derive::ValueStruct;

    #[derive(Debug, ValueStruct, Clone)]
    struct SimpleStrValueStruct(String);

    #[derive(Debug, ValueStruct, Clone)]
    struct StdStrValueStruct(std::string::String);

    #[derive(Debug, ValueStruct, Clone)]
    struct SimpleIntValueStruct(u8);

    #[derive(ValueStruct)]
    struct UserId(String);

    #[test]
    fn create_str_value_struct() {
        let s1 : SimpleStrValueStruct = String::from("Hey").into();
        assert_eq!(s1.value(), "Hey");

        let s12: SimpleStrValueStruct = "Hey".into();
        assert_eq!(s12.value(), "Hey");

        let s13 = SimpleStrValueStruct::from(s12);
        assert_eq!(s13.value(), "Hey");
    }

    #[test]
    fn create_std_str_value_struct() {
        let s1 : SimpleStrValueStruct = String::from("Hey").into();
        assert_eq!(s1.value(), "Hey");
    }

    #[test]
    fn create_int_value_struct() {
        let i1 : SimpleIntValueStruct = 1u8.into();
        assert_eq!(i1.value(), 1u8);
    }

    #[test]
    fn create_example_struct() {
        let uid : UserId = "my-uid".into();
        assert_eq!(uid.value(), "my-uid");
    }
}
