use std::{
    fmt,
    string::FromUtf8Error,
};

#[derive(Debug, Clone)]
pub struct ClipboardError {
    message: String,
}

impl From<String> for ClipboardError {
    fn from(message: String) -> Self {
        Self { message }
    }
}
impl From<&str> for ClipboardError {
    fn from(message: &str) -> Self {
        Self { message: message.to_owned() }
    }
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "clipboard error: {}", self.message)
    }
}

impl From<FromUtf8Error> for ClipboardError {
    fn from(_: FromUtf8Error) -> Self {
        ClipboardError::from("error interpreting as UTF8".to_string())
    }
}
