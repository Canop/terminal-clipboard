use {
    crate::errors::ClipboardError,
    std::time::Duration,
    x11_clipboard::{self, Clipboard},
};

impl From<x11_clipboard::error::Error> for ClipboardError {
    fn from(x11_error: x11_clipboard::error::Error) -> Self {
        ClipboardError::from(format!("X11 clipboard error : {}", x11_error))
    }
}

pub fn get_string() -> Result<String, ClipboardError> {
    let clipboard = Clipboard::new()?;
    Ok(String::from_utf8(clipboard.load(
        clipboard.getter.atoms.clipboard,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_secs(3),
    )?)?)
}

pub fn set_string<S: AsRef<str>>(s: S) -> Result<(), ClipboardError> {
    let clipboard = Clipboard::new()?;
    Ok(clipboard.store(
        clipboard.setter.atoms.clipboard,
        clipboard.setter.atoms.utf8_string,
        s.as_ref(),
    )?)
}
