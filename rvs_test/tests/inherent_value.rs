use rvstruct::ValueStruct;
use std::sync::OnceLock;

#[derive(ValueStruct)]
struct Password(String);

impl Password {
    /// Shadows `ValueStruct::value` so that the secret is never shown.
    fn value(&self) -> &String {
        static REDACTED: OnceLock<String> = OnceLock::new();
        REDACTED.get_or_init(|| "<redacted>".to_owned())
    }
}

#[test]
fn display_and_as_ref_use_inherent_value() {
    let password: Password = "hunter2".into();
    assert_eq!(password.to_string(), "<redacted>");
    assert_eq!(password.as_ref(), "<redacted>");
    assert_eq!(ValueStruct::value(&password), "hunter2");
}
