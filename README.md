

*Warning: still experimental*

**terminal-clipboard** is a cross-platform clipboard library focused on strings copying and pasting for terminal applications:

* it's tested on macos, linux, windows
* it doesn't support Wayland (because you're in the terminal)
* it doesn't handle other types of objects than strings
* it doesn't handle non UTF8 strings

If this doesn't match your requirements, don't hesitate to search for another crate: there are many ones with other goals.

**terminal-clipboard** is only a facade over other, specialized, clipboard crates so have a look at its own dependencies if you want a system specific clipboard or capabilities going past just text.

# Installation

```
[dependencies]
terminal_clipboard = "0.1"
```

On linux you'll need to have `xorg-dev` and `libxcb-composite0-dev` to compile.

On Debian and Ubuntu you can install them with

	sudo apt install xorg-dev libxcb-composite0-dev

# Usage


```
use terminal_clipboard;
let test = "TEST";
terminal_clipboard::set_string(test).unwrap();
assert_eq!(test, terminal_clipboard::get_string().unwrap());
```

