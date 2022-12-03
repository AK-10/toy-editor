use libc;
use libc::{
    BRKINT, CS8, CSIZE, ECHO, ECHONL, ICANON, ICRNL, IEXTEN, IGNBRK, IGNCR, INLCR, ISIG, ISTRIP,
    IXON, OPOST, PARENB, PARMRK,
};
use std::io::{stdin, Read};

fn main() {
    // 現在のterminal情報を取得
    let mut term = unsafe {
        let mut term = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut term);

        term
    };
    let origin_term = term.clone();

    // rawモードに切り替え
    // man cfmakerawに色々書いてある
    unsafe {
        println!("makeraw");
        term.c_iflag &= !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
        term.c_oflag &= !OPOST;
        term.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        term.c_cflag &= !(CSIZE | PARENB);
        term.c_cflag |= CS8;

        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);

        // 何故かうまく行かない
        //
        // libc::cfmakeraw(&mut term);
    }

    while let Some(b) = stdin().bytes().next() {
        // control + qで離脱
        if let Ok(17) = b {
            break;
        }
        println!("b: {:?}", b);
    }

    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &origin_term);
    }
}
