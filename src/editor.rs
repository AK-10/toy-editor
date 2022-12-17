use crate::{
    renderer::Renderer,
    text::Text,
    pane::Pane,
    terminal::Terminal
};

use std::cell::RefCell;
use std::rc::Rc;
use std::error;

fn control_char(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

pub struct Editor {
    text: Rc<RefCell<Text>>,
    renderer: Renderer,
    pane: Pane,
    reader: Terminal

}

impl Editor {
    pub fn new(text: Text) -> Self {
        let text = Rc::new(RefCell::new(text));
        let renderer = Renderer::new(Rc::clone(&text));
        let pane = Pane::new(Rc::clone(&text));
        let reader = Terminal::new();

        Self {
            text,
            renderer,
            pane,
            reader
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.reader.enable_raw_mode();
        self.renderer.render(true)?;

        loop {
            match self.reader.read_key()? {
                // 終了
                b if b == control_char('q') => break,
                // 左
                b if b == control_char('h') => {
                    let _ = self.pane.decrement_col();
                }
                // 下
                b if b == control_char('j') => {
                    let _ = self.pane.increment_row();
                }
                // 上
                b if b == control_char('k') => {
                    let _ = self.pane.decrement_row();
                }
                // 右
                b if b == control_char('l') => {
                    let _ = self.pane.increment_col();
                }
                b if (b as char).is_alphabetic() => {
                    self.insert(b)?;
                }
                _ => continue
            }

            self.renderer.render(false)?;
            self.renderer.move_cursor(self.pane.current_pos())?;
        }

        Ok(())
    }

    pub fn insert(&mut self, b: u8) -> Result<(), Box<dyn error::Error>> {
        self.text.borrow_mut().insert(self.pane.current_pos(), b as char)?;
        self.pane.increment_col()?;

        Ok(())
    }
}
