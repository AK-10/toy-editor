use crate::text::Text;
use std::{error, fmt};
use std::io::{self, stdout, Write};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Renderer {
    text: Rc<RefCell<Text>>,
}

#[derive(Debug)]
pub enum Error {
    RenderError(String)
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::RenderError(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::RenderError(msg) => {
                write!(f, "render error: {}", msg)
            }
        }
    }
}

impl Renderer {
    pub fn new(text: Rc<RefCell<Text>>) -> Self {
        Self {
            text
        }
    }

    pub fn render(&self, is_first: bool) -> Result<(), Error> {
        // 画面全体をクリア
        print!("\x1b[2J");
        // カーソルを左上(ホームポジション)に移動
        print!("\x1b[H");

        for (i, row) in self.text.borrow().rows().iter().enumerate() {
            print!("{}", row);
            if i != self.text.borrow().rows().len() - 1 {
                print!("\x1b[E");
            }
        }
        // カーソルを左上(ホームポジション)に移動(起動時のみ)
        if is_first {
            print!("\x1b[H");
        }

        stdout().flush()?;

        Ok(())
    }

    pub fn move_cursor(&self, pos: (usize, usize)) -> Result<(), Error> {
        // カーソルの位置を変更する
        // \x1b[{行};{列}H
        // ターミナルは(1, 1)から始まる
        // 一方で受け取るposは0を基準とした値になるため、+1したものを出力する
        let sequence = format!("\x1b[{};{}H", pos.0 + 1, pos.1 + 1);

        print!("{}", sequence);
        stdout().flush()?;

        Ok(())
    }
}
