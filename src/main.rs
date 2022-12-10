use std::io::{stdin, Read, stdout, Write};
use std::env;

use toy_editor::{
    terminal::Terminal,
    text::Text
};

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("expected file path to first arg, but nothing");

    let text = Text::from_path(path).expect("expected open file, and read content");

    let mut term = Terminal::new();
    term.enable_raw_mode();

    // 画面全体をクリア
    print!("\x1b[2J");
    // カーソルを左上(ホームポジション)に移動
    print!("\x1b[H");
    let _ = stdout().flush();

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
