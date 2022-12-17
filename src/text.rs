use std::fs::File;
use std::io::{self, Read};
use std::{error, fmt};

pub type Row = String;

#[derive(Debug)]
pub struct Text {
    path: String,
    rows: Vec<Row>
}

#[derive(Debug)]
pub enum Error {
    OpenError(String),
    ModifyError(String)
}

pub enum DeleteStatus {
    Nop,
    DeleteChar,
    DeleteRow(usize, usize),
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::OpenError(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::OpenError(msg) => write!(f, "open file error: {}", msg),
            Self::ModifyError(msg) => write!(f, "modify text error: {}", msg)

        }
    }
}

impl Text {
    pub fn from_path(path: String) -> Result<Self, Error> {
        let mut file = File::open(&path)?;
        let mut buf = String::with_capacity(4096);
        file.read_to_string(&mut buf)?;

        let rows: Vec<String> = buf
            .lines()
            .map(Row::from)
            .collect();

        Ok(Self {
            path,
            rows
        })
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn insert(&mut self, pos: (usize, usize), ch: char) -> Result<(), Error> {
        self.validate_position(pos)?;

        self.rows[pos.0].insert(pos.1, ch);
        Ok(())
    }

    //
    pub fn delete(&mut self, pos: (usize, usize)) -> Result<DeleteStatus, Error> {
        self.validate_position(pos)?;
        match pos {
            (0, 0) => {
                // nop
                // 削除対象がないため、何もしない
                Ok(DeleteStatus::Nop)
            }
            (_, 0) => {
                // 先頭の削除は、前の行と今の行の連結を行う
                let removed_row = self.rows.remove(pos.0);
                let prev_row = &mut self.rows[pos.0 - 1];
                let prev_row_len = prev_row.len();
                prev_row.push_str(&removed_row);

                // 連結したあとの先頭の位置を返す
                Ok(DeleteStatus::DeleteRow(pos.0 - 1, prev_row_len))
            }
            _ => {
                self.rows[pos.0].remove(pos.1 - 1);
                Ok(DeleteStatus::DeleteChar)
            }
        }
    }

    fn validate_position(&self, pos: (usize, usize)) -> Result<(), Error> {
        if pos.0 >= self.rows.len() {
            let msg = format!("row is out of range. rows len: {}, row pos: {}", self.rows.len(), pos.0);
            return Err(Error::ModifyError(msg.into()));
        }

        let row = &self.rows[pos.0];
        if pos.1 > row.len() {
            let msg = format!("col is out of range. row len: {}, col pos: {}", row.len(), pos.0);
            return Err(Error::ModifyError(msg.into()));
        }

        Ok(())
    }
}
