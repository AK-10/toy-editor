use crate::text::Text;

// positionは(0, 0)から始まる
// 左上が0, 0
pub struct Position {
    row: usize,
    col: usize
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {
            row: value.0,
            col: value.1
        }
    }
}

impl Position {
    pub fn to_pair(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

pub struct Pane<'a> {
    position: Position,
    text: &'a Text
}

pub enum Error {
    ModifyPositionError(String)
}

impl<'a> Pane<'a> {
    pub fn new(text: &'a Text) -> Self {
        Self {
            position: (0, 0).into(),
            text
        }
    }

    pub fn increment_row(&mut self) -> Result<(usize, usize), Error> {
        if self.position.row < self.text.rows().len() - 1 {
            // 一つ次の行の文字数が今の行の文字数より少ない場合,
            // 一つ次の行の末尾にカーソルが移動するようにする
            let next_row_len = self.text.rows()[self.position.row + 1].len();
            if self.position.col > next_row_len {
                self.position.col = next_row_len;
            }
            self.position.row += 1;
            Ok(self.position.to_pair())
        } else {
            Err(Error::ModifyPositionError("increment_row is unable at the cursor on last of row".into()))
        }
    }

    pub fn decrement_row(&mut self) -> Result<(usize, usize), Error> {
        if self.position.row > 0 {
            // 一つ前の行の文字数が今の行の文字数より少ない場合,
            // 一つ前の行の末尾にカーソルが移動するようにする
            let prev_row_len = self.text.rows()[self.position.row - 1].len();
            if self.position.col > prev_row_len {
                self.position.col = prev_row_len;
            }
            self.position.row -= 1;
            Ok(self.position.to_pair())
        } else {
            Err(Error::ModifyPositionError("decrement_row is unable at the cursor on first of row".into()))
        }
    }

    pub fn increment_col(&mut self) -> Result<(usize, usize), Error> {
        let current_col_size = self.text.rows()[self.position.row].len();
        if self.position.col < current_col_size {
            self.position.col += 1;
            Ok(self.position.to_pair())
        } else {
            Err(Error::ModifyPositionError("increment_col is unable at the cursor on last of col".into()))
        }
    }

    pub fn decrement_col(&mut self) -> Result<(usize, usize), Error> {
        if self.position.col > 0 {
            self.position.col -= 1;
            Ok(self.position.to_pair())
        } else {
            Err(Error::ModifyPositionError("decrement_col is unable at the cursor on first of col".into()))
        }
    }
}
