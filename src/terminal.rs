use libc::{
    self,
    BRKINT, CS8, CSIZE, ECHO, ECHONL, ICANON, ICRNL, IEXTEN, IGNBRK, IGNCR, INLCR, ISIG, ISTRIP,
    IXON, OPOST, PARENB, PARMRK
};

pub struct Terminal {
    original_term: libc::termios
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
            //
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
}

impl Drop for Terminal {
    // 終了時に起動前のターミナルの状態に戻す
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.original_term);
        }
    }
}
