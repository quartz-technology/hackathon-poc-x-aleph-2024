use super::EntryType;

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait Entry: std::fmt::Debug + AToAny {
    fn path(&self) -> &str;

    fn set_path(&mut self, path: String);

    fn entry_type(&self) -> EntryType;
}