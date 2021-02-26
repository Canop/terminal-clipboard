/*!

**terminal-clipboard** is a cross-platform clipboard library focused on strings copy and paste in the terminal:

* it's cross-compilation friendly
* it's tested on macos, linux, windows and termux
* it doesn't support Wayland (because you're in the terminal)
* it doesn't handle other types of objects than strings
* it doesn't handle non UTF8 strings

It exposes only two functions, one for reading the clipboard as a string, another one to fill it from a string:

```
use terminal_clipboard;
terminal_clipboard::set_string("test").unwrap();
assert_eq!("test", terminal_clipboard::get_string().unwrap());
```

*/

mod errors;
pub use errors::ClipboardError;

// #[cfg(target_os="macos")]
// pub mod macos;
// #[cfg(target_os="macos")]
// pub use macos::{get_string, set_string};

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::{get_string, set_string};

#[cfg(target_os = "android")]
mod termux;
#[cfg(target_os = "android")]
pub use termux::{get_string, set_string};

#[cfg(not(any(target_os="windows",target_os="android")))]
mod x11;
#[cfg(not(any(target_os="windows",target_os="android")))]
pub use x11::{get_string, set_string};


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
