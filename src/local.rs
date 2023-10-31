use {
    crate::{
        Clipboard,
        errors::ClipboardError,
    },
};

/// A clipboard with no access to outside the application.
#[derive(Default)]
pub struct LocalClipboard {
    content: String,
}

impl LocalClipboard {
    pub fn new() -> LocalClipboard {
        Self {
            content: String::new(),
        }
    }
}

impl Clipboard for LocalClipboard {

    fn get_type(&self) -> &'static str {
        "Local"
    }

    fn get_string(&self) -> Result<String, ClipboardError> {
        let mut copy = String::with_capacity(self.content.len());
        copy.push_str(&self.content);
        Ok(copy)
    }

    fn set_string(&mut self, s: &str) -> Result<(), ClipboardError> {
        self.content.clear();
        self.content.push_str(s);
        Ok(())
    }
}

#[test]
fn test_local_clipboard() {
    let mut clipboard = LocalClipboard::new();
    assert_eq!(clipboard.get_type(), "Local");
    clipboard.set_string("test local").unwrap();
    assert_eq!(clipboard.get_string().unwrap(), "test local".to_string());
}
