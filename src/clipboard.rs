use {
    crate::errors::ClipboardError,
};

pub trait Clipboard {

    fn get_type(&self) -> &'static str;

    fn get_string(&self) -> Result<String, ClipboardError>;

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError>;
}
