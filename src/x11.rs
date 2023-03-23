use {
    crate::{
        Clipboard,
        errors::ClipboardError,
    },
    std::time::Duration,
};

impl From<x11_clipboard::error::Error> for ClipboardError {
    fn from(x11_error: x11_clipboard::error::Error) -> Self {
        ClipboardError::from(format!("X11 clipboard error : {}", x11_error))
    }
}

pub struct X11Clipboard {
    backend: x11_clipboard::Clipboard,
}

impl X11Clipboard {
    pub fn new() -> Result<X11Clipboard, ClipboardError> {
        let backend = x11_clipboard::Clipboard::new()?;
        Ok(Self { backend })
    }
    /// return a X11 clipboard but only if it has been verified to
    /// be correctly working
    pub fn verified() -> Result<X11Clipboard, ClipboardError> {
        let backend = x11_clipboard::Clipboard::new()?;
        let mut clipboard = Self { backend };
        let previous = clipboard.get_string()?; // saving the old value
        let test = "test X11";
        clipboard.set_string(test)?;
        let res = clipboard.get_string()?;
        clipboard.set_string(&previous)?;
        if res == *test {
            Ok(clipboard)
        } else {
            Err(ClipboardError::from("non compliand round trip"))
        }
    }
}

impl Clipboard for X11Clipboard {

    fn get_type(&self) -> &'static str {
        "X11"
    }

    fn get_string(&self) -> Result<String, ClipboardError> {
        Ok(String::from_utf8(self.backend.load(
            self.backend.getter.atoms.clipboard,
            self.backend.getter.atoms.utf8_string,
            self.backend.getter.atoms.property,
            Duration::from_secs(2),
        )?)?)
    }

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError> {
        Ok(self.backend.store(
            self.backend.setter.atoms.clipboard,
            self.backend.setter.atoms.utf8_string,
            s,
        )?)
    }
}
