use std::io::{stdin, Read};
use std::env;

use toy_editor::{
    text::Text,
    editor::Editor
};


fn main() {
    let path = env::args()
        .nth(1)
        .expect("expected file path to first arg, but nothing");

    let text = Text::from_path(path).expect("expected open file, and read content");
    let mut editor = Editor::new(text);
    let _ = editor.run();
}
