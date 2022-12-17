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
        if pos.0 > self.rows.len() {
            let msg = format!("out of range. rows len: {}, pos: {}", self.rows.len(), pos.0);
            return Err(Error::ModifyError(msg.into()));
        }
        self.rows[pos.0].insert(pos.1, ch);
        Ok(())
    }
}
