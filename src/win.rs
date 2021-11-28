use {
    crate::{
        Clipboard,
        errors::ClipboardError,
    },
};

pub struct WinClipboard {}

impl WinClipboard {
    pub fn new() -> WinClipboard {
        Self {}
    }
}

impl Clipboard for WinClipboard {

    fn get_type(&self) -> &'static str {
        "Windows"
    }

    fn get_string(&self) -> Result<String, ClipboardError> {
        clipboard_win::get_clipboard_string()
            .map_err(|e| ClipboardError::from(format!("Windows clipboard error : {}", e,)))
    }

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError> {
        clipboard_win::set_clipboard_string(s)
            .map_err(|e| ClipboardError::from(format!("Windows clipboard error : {}", e,)))
    }

}
