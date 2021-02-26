use {
    crate::errors::ClipboardError,
    termux_clipboard::TermuxClipboardError,
};

impl From<TermuxClipboardError> for ClipboardError {
    fn from(termux_error: TermuxClipboardError) -> Self {
        ClipboardError::from(termux_error.to_string())
    }
}

pub fn get_string() -> Result<String, ClipboardError> {
    Ok(termux_clipboard::get_string()?)
}

pub fn set_string<S: AsRef<str>>(s: S) -> Result<(), ClipboardError> {
    Ok(termux_clipboard::set_string(s)?)
}

