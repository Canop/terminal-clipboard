use std::{fmt, string::FromUtf8Error};

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
        Self {
            message: message.to_owned(),
        }
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

#[test]
fn test_from_string() {
    let message = "test error message".to_string();
    let err = ClipboardError::from(message.clone());
    assert_eq!(err.message, message);
}

#[test]
fn test_from_str() {
    let message = "test error message";
    let err = ClipboardError::from(message);
    assert_eq!(err.message, message.to_string());
}

#[test]
fn test_display() {
    let message = "test error message";
    let err = ClipboardError::from(message);
    let display = format!("{}", err);
    assert_eq!(display, format!("clipboard error: {}", message));
}

#[test]
fn test_from_utf8_error() {
    let invalid_utf8_sequence = vec![0xC3, 0x28];
    let utf8_error = String::from_utf8(invalid_utf8_sequence).unwrap_err();
    let err: ClipboardError = utf8_error.into();
    assert_eq!(err.message, "error interpreting as UTF8".to_string());
}

#[test]
fn test_debug() {
    let message = "test error message";
    let err = ClipboardError::from(message);
    let debug = format!("{:?}", err);
    assert_eq!(
        debug,
        format!("ClipboardError {{ message: \"{}\" }}", message)
    );
}
