use crate::{
    errors::ClipboardError,
    Clipboard,
};

pub struct MacClipboard {
    backend: clipboard_macos::Clipboard,
}

impl MacClipboard {
    pub fn new() -> Result<MacClipboard, ClipboardError> {
        clipboard_macos::Clipboard::new()
            .map(|backend| Self { backend })
            .map_err(|e| ClipboardError::from(format!("macOS clipboard error : {}", e)))
    }

    pub fn verified() -> Result<MacClipboard, ClipboardError> {
        let mut clipboard = Self::new()?;
        let previous = clipboard.get_string()?; // saving the old value
        let test = "test macOS";
        clipboard.set_string(test)?;
        let res = clipboard.get_string()?;
        clipboard.set_string(&previous)?;
        if res == test.to_string() {
            Ok(clipboard)
        } else {
            Err(ClipboardError::from("non compliand round trip"))
        }
    }
}

impl Clipboard for MacClipboard {
    fn get_type(&self) -> &'static str {
        "macOS"
    }

    fn get_string(&self) -> Result<String, ClipboardError> {
        self.backend
            .read()
            .map_err(|e| ClipboardError::from(format!("macOS clipboard error : {}", e,)))
    }

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError> {
        self.backend
            .write(s.to_string())
            .map_err(|e| ClipboardError::from(format!("macOS clipboard error : {}", e,)))
    }
}
