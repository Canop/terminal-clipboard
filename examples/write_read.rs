//! run with
//!     cargo run --example write_read

fn main() {
    println!("Clipboard backend type: {:?}", terminal_clipboard::get_type());
    println!(
        "Initial content of the clipboard: {:?}",
        terminal_clipboard::get_string().unwrap(),
    );
    let s = "TEST";
    println!("Writing {:?} in the clipboard", s);
    terminal_clipboard::set_string(s).unwrap();
    println!(
        "New content of the clipboard: {:?}",
        terminal_clipboard::get_string().unwrap(),
    );
}
