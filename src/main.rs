use std::io::{stdin, Read};

use toy_editor::terminal::Terminal;

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn main() {
    let mut term = Terminal::new();
    term.enable_raw_mode();

    while let Some(b) = stdin().bytes().next() {
        println!("b: {:?}", b);
        // control + qで離脱
        if let Ok(b) = b {
            if b == control_char('q') {
                break
            }
        }
    }
}
