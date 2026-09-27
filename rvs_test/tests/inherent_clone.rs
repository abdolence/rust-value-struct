use rvstruct::ValueStruct;

/// Has a `clone` method of its own and no `Clone` impl.
#[derive(Debug, PartialEq)]
struct Handle(u32);

impl Handle {
    fn clone(&self) -> Self {
        Handle(self.0 + 1)
    }
}

#[derive(Debug, ValueStruct)]
struct HandleId(Handle);

#[test]
fn from_reference_uses_inherent_clone() {
    let raw = Handle(1);
    let id = HandleId::from(&raw);
    assert_eq!(id.value(), &Handle(2));
}
