use std::io::{stdin, Read, stdout, Write};
use std::env;
use std::fs::File;

use toy_editor::terminal::Terminal;

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("expected file path to first arg, but nothing");

    let file = match File::open(path) {
        Ok(ref mut f) => {
            let mut buf = String::with_capacity(4096);
            if let Ok(_) = f.read_to_string(&mut buf) {
                buf
            } else {
                std::process::exit(1)
            }
        }
        Err(_) => std::process::exit(1)
    };

    let text: Vec<String> = file
        .lines()
        .map(String::from)
        .collect();

    let mut term = Terminal::new();
    term.enable_raw_mode();

    text.iter().enumerate().for_each(|(i, row)| {
        print!("{}", row);
        if i != text.len() - 1 {
            print!("\x1b[E");
        }
        let _ = stdout().flush();
    });

    while let Some(b) = stdin().bytes().next() {
        // control + qで離脱
        if let Ok(b) = b {
            if b == control_char('q') {
                break
            }
        }
    }


}
