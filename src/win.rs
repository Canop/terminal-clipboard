use {crate::errors::ClipboardError, clipboard_win};

pub fn get_string() -> Result<String, ClipboardError> {
    clipboard_win::get_clipboard_string()
        .map_err(|e| ClipboardError::from(format!("Windows clipboard error : {}", e,)))
}

pub fn set_string<S: AsRef<str>>(s: S) -> Result<(), ClipboardError> {
    clipboard_win::set_clipboard_string(s.as_ref())
        .map_err(|e| ClipboardError::from(format!("Windows clipboard error : {}", e,)))
}
