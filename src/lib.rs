/*!

**terminal-clipboard** is a cross-platform clipboard library focused on strings copying and pasting for terminal applications:

* it's tested on linux, windows and Android (Termux)
* it doesn't handle other types of objects than strings
* it doesn't handle non UTF8 strings

# Usage

```
terminal_clipboard::set_string("test").unwrap();
assert_eq!("test", terminal_clipboard::get_string().unwrap());
```

# Supported platforms

The implementation is currently chosen from the "target_os" part of the compilation target.

## Android (Termux)

The current implementation will defer to Termux API facilities to access the Android clipboard, and won't work if the Termux API isn't available at runtime.

If you know of solutions to access the Android clipboard without Termux, please open an issue.

## Linux

If a unix-like target is detected and the "termux" feature isn't enabled, terminal-clipboard uses the [x11-clipboard](https://crates.io/crates/x11-clipboard) crate.

You'll need to have `xorg-dev` and `libxcb-composite0-dev` to compile.

On Debian and Ubuntu you can install them with

```bash
sudo apt install xorg-dev libxcb-composite0-dev
```

## Windows

If the compilation target is "windows", terminal-clipboard uses the [clipboard-win](https://crates.io/crates/clipboard-win) crate. If you're only interested in this platform, you should use this crate directly.


```
use terminal_clipboard;
terminal_clipboard::set_string("test").unwrap();
assert_eq!("test", terminal_clipboard::get_string().unwrap());
```

*/

mod clipboard;
mod errors;
mod local;

pub use {
    clipboard::Clipboard,
    errors::ClipboardError,
    local::LocalClipboard,
};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacClipboard;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::WinClipboard;

#[cfg(target_os = "android")]
mod termux;
#[cfg(target_os = "android")]
pub use termux::TermuxClipboard;

#[cfg(not(any(target_os="windows",target_os="android",target_os="macos")))]
mod x11;
#[cfg(not(any(target_os="windows",target_os="android",target_os="macos")))]
pub use x11::X11Clipboard;

use {
    once_cell::sync::Lazy,
    std::sync::Mutex,
};

static CLIPBOARD: Lazy<Mutex<Box<dyn Clipboard + Send>>> = Lazy::new(|| Mutex::new(new_clipboard()));

/// Build a new clipboard.
///
/// It's recommended to not use this directly but the get_string
/// and set_string global functions instead, as those functions
/// ensure the lazy static initialization of the clipboard.
pub fn new_clipboard() -> Box<dyn Clipboard + Send> {
    #[cfg(target_os = "windows")]
    {
        // To my knowledge
        return Box::new(WinClipboard::new());
    }

    #[cfg(target_os = "macos")]
    {
        // only use MacClipboard after it is verified.
        if let Ok(clipboard) = MacClipboard::verified() {
            return Box::new(clipboard);
        }
    }

    #[cfg(target_os = "android")]
    {
        // we'll use the Termux clipboard, but only after having
        // checked it works. It fails for example when the
        // Termux API isn't available on the device
        if let Ok(clipboard) =TermuxClipboard::verified() {
            return Box::new(clipboard);
        }
    }

    #[cfg(not(any(target_os="windows",target_os="android",target_os="macos")))]
    {
        // we'll use the X11 clipboard, but only after having
        // checked it works. As nobody understants X11 anyway,
        // I won't try to pretend I know when it works and when
        // it doesn't
        if let Ok(clipboard) = X11Clipboard::verified() {
            return Box::new(clipboard);
        }
    }

    // when everything else failed, use a local clipboard
    #[allow(unreachable_code)]
    Box::new(LocalClipboard::new())
}

/// Return the type of the Clipboard, for example
/// "X11", "Windows", "Local", or "Termux"
pub fn get_type() -> &'static str {
    CLIPBOARD.lock().unwrap().get_type()
}

/// Return the content of the clipboard
pub fn get_string() -> Result<String, ClipboardError> {
    CLIPBOARD.lock().unwrap().get_string()
}

/// Fill the clipboard with the given string
pub fn set_string<S: AsRef<str>>(s: S) -> Result<(), ClipboardError> {
    CLIPBOARD.lock().unwrap().set_string(s.as_ref())
}

// Those tests are the same than doc tests but they must be
// kept separate because cargo-cross doesn't run doc tests
// (see https://github.com/rust-embedded/cross/issues/225)
#[cfg(test)]
mod clipboard_tests {

    use super::*;

    #[test]
    fn write_read() {
        let test = "TEST";
        set_string(test).unwrap();
        assert_eq!(test, get_string().unwrap());
    }
}
