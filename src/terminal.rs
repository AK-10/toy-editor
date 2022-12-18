use std::io::{stdin, Read};
use std::{error, fmt};

use libc::{
    self,
    BRKINT, CS8, CSIZE, ECHO, ECHONL, ICANON, ICRNL, IEXTEN, IGNBRK, IGNCR, INLCR, ISIG, ISTRIP,
    IXON, OPOST, PARENB, PARMRK
};

pub struct Terminal {
    original_term: libc::termios
}

#[derive(Debug)]
pub enum Error {
    ReadError(String),
}
impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ReadError(msg) => {
                write!(f, "render error: {}", msg)
            }
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        // 現在のterminal情報を取得
        let term = unsafe {
            let mut term = std::mem::zeroed();
            libc::tcgetattr(libc::STDIN_FILENO, &mut term);

            term
        };

        Self {
            original_term: term
        }
    }

    pub fn enable_raw_mode(&mut self) {
        let mut term = self.original_term.clone();
        unsafe {
            // 何故かうまく行かない
            // libc::cfmakeraw(&mut term);

            term.c_iflag &= !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
            term.c_oflag &= !OPOST;
            term.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
            term.c_cflag &= !(CSIZE | PARENB);
            term.c_cflag |= CS8;

            // terminalをrawモードに更新
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);
        }
    }

    pub fn read_key(&self) -> Result<u8, Error> {
        match stdin().bytes().next() {
            Some(Ok(b)) => Ok(b),
            Some(Err(e)) => Err(Error::ReadError(e.to_string())),
            None => Err(Error::ReadError("failed reading key input".into()))
        }
    }
}

impl Drop for Terminal {
    // 終了時に起動前のターミナルの状態に戻す
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.original_term);
        }
    }
}
