
fn main() {
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
