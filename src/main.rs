use std::io::{stdin, Read};
use std::env;

use toy_editor::{
    terminal::Terminal,
    text::Text,
    renderer::Renderer
};

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("expected file path to first arg, but nothing");

    let text = Text::from_path(path).expect("expected open file, and read content");
    let renderer = Renderer::new(text);
    renderer.render().expect("expect render");

    let mut term = Terminal::new();
    term.enable_raw_mode();

    while let Some(b) = stdin().bytes().next() {
        // control + qで離脱
        if let Ok(b) = b {
            if b == control_char('q') {
                break
            } else if b == control_char('h') {
                println!("left");
            } else if b == control_char('j') {
                println!("down");
            } else if b == control_char('k') {
                println!("up");
            } else if b == control_char('l') {
                println!("right");
            }
        }
    }
}
