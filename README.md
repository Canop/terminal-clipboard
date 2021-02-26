# terminal-clipboard

[![CI][s3]][l3] [![MIT][s2]][l2] [![Latest Version][s1]][l1] [![Chat on Miaou][s3]][l3]

[s1]: https://img.shields.io/crates/v/terminal-clipboard.svg
[l1]: https://crates.io/crates/terminal-clipboard

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://miaou.dystroy.org/static/shields/room.svg
[l3]: https://miaou.dystroy.org/3490?terminal-clipboard


**terminal-clipboard** is a cross-platform clipboard library focused on strings copying and pasting for terminal applications:

* it's tested on macos, linux, windows and Android (termux)
* it doesn't support Wayland (because you're in the terminal)
* it doesn't handle other types of objects than strings
* it doesn't handle non UTF8 strings

If this doesn't match your requirements, don't hesitate to search for another crate: there are many ones with other goals.

# Installation

```
[dependencies]
terminal_clipboard = "0.2"
```

# Usage

```
use terminal_clipboard;
terminal_clipboard::set_string("test").unwrap();
assert_eq!("test", terminal_clipboard::get_string().unwrap());
```

# Supported platforms

## Termux

A specific implementation is available, defering to the Termux API facilities to access the Android clipboard.

You may either enable it statically (at compile time), or choose it dynamically (which may be useful when the same binary is used in several contexts).

### Enable the Termux API at compile time

This is done by compiling with the "termux" feature:

```TOML
terminal-clipboard = { version="0.2", features=["termux"] }
```

In such a case, `terminal_clipboard::get_string` and `terminal_clipboard::set_string` are the Termux implementations.

### Call dynamically

If you don't enable the "termux" feature, `terminal_clipboard::get_string` and `terminal_clipboard::set_string` are the linux or windows implementations and you can decide to call the termux one by calling  `terminal_clipboard::termux::get_string` and `terminal_clipboard::termux::set_string`.


## Linux

If a unix-like target is detected and the "termux" feature isn't enabled, terminal-clipboard uses the [x11-clipboard](https://crates.io/crates/x11-clipboard) crate. If you're only interested in this platform, you should use this crate directly.

You'll need to have `xorg-dev` and `libxcb-composite0-dev` to compile.

On Debian and Ubuntu you can install them with

	sudo apt install xorg-dev libxcb-composite0-dev

## Windows

If the compilation target is "windows", terminal-clipboard uses the [clipboard-win](https://crates.io/crates/clipboard-win) crate. If you're only interested in this platform, you should use this crate directly.

