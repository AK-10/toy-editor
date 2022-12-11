use std::io::{stdin, Read};
use std::env;

use toy_editor::{
    terminal::Terminal,
    text::Text,
    renderer::Renderer,
    pane::Pane
};

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("expected file path to first arg, but nothing");

    let text = Text::from_path(path).expect("expected open file, and read content");
    let mut pane = Pane::new(&text);
    let renderer = Renderer::new(&text);
    renderer.render().expect("expect render");

    let mut term = Terminal::new();
    term.enable_raw_mode();

    while let Some(b) = stdin().bytes().next() {
        // control + qで離脱
        if let Ok(b) = b {
            if b == control_char('q') {
                break
            } else if b == control_char('h') {
                if let Ok(pos) = pane.decrement_col() {
                    let _ = renderer.move_cursor(pos);
                }
            } else if b == control_char('j') {
                if let Ok(pos) = pane.increment_row() {
                    let _ = renderer.move_cursor(pos);
                }
            } else if b == control_char('k') {
                if let Ok(pos) = pane.decrement_row() {
                    let _ = renderer.move_cursor(pos);
                }
            } else if b == control_char('l') {
                if let Ok(pos) = pane.increment_col() {
                    let _ = renderer.move_cursor(pos);
                }
            }
        }
    }
}
