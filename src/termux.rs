use {
    crate::{
        Clipboard,
        errors::ClipboardError,
    },
    termux_clipboard::TermuxClipboardError,
};

impl From<TermuxClipboardError> for ClipboardError {
    fn from(termux_error: TermuxClipboardError) -> Self {
        ClipboardError::from(termux_error.to_string())
    }
}

pub struct TermuxClipboard {}

impl TermuxClipboard {
    pub fn new() -> TermuxClipboard {
        Self {}
    }
    pub fn verified() -> Result<TermuxClipboard, ClipboardError> {
        let mut clipboard = Self {};
        let test = "test Termux";
        clipboard.set_string(test)?;
        let res = clipboard.get_string()?;
        if res == test.to_string() {
            Ok(clipboard)
        } else {
            Err(ClipboardError::from("non compliand round trip"))
        }
    }
}

impl Clipboard for TermuxClipboard {

    fn get_type(&self) -> &'static str {
        "Termux"
    }

    fn get_string(&self) -> Result<String, ClipboardError> {
        Ok(termux_clipboard::get_string()?)
    }

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError> {
        Ok(termux_clipboard::set_string(s)?)
    }

}


