# terminal-clipboard

[![CI][s3]][l3] [![MIT][s2]][l2] [![Latest Version][s1]][l1] [![Chat on Miaou][s3]][l3]

[s1]: https://img.shields.io/crates/v/terminal-clipboard.svg
[l1]: https://crates.io/crates/terminal-clipboard

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://miaou.dystroy.org/static/shields/room.svg
[l3]: https://miaou.dystroy.org/3490?terminal-clipboard


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

	sudo apt install xorg-dev libxcb-composite0-dev

## Windows

If the compilation target is "windows", terminal-clipboard uses the [clipboard-win](https://crates.io/crates/clipboard-win) crate. If you're only interested in this platform, you should use this crate directly.


