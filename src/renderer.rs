use crate::text::Text;
use std::{error, fmt};
use std::io::{self, stdout, Write};

pub struct Renderer {
    text: Text,
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
    pub fn new(text: Text) -> Self {
        Self {
            text
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        // 画面全体をクリア
        print!("\x1b[2J");
        // カーソルを左上(ホームポジション)に移動
        print!("\x1b[H");
        stdout().flush()?;

        for (i, row) in self.text.rows().iter().enumerate() {
            print!("{}", row);
            if i != self.text.rows().len() - 1 {
                print!("\x1b[E");
            }
            stdout().flush()?;
        }

        Ok(())
    }
}
