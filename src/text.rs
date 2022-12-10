use std::fs::File;
use std::io::Read;
use std::{error, fmt};

type Row = String;

#[derive(Debug)]
pub struct Text {
    path: String,
    text: Vec<Row>
}

#[derive(Debug)]
pub enum Error {
    OpenError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::OpenError(msg) => {
                write!(f, "text open error: {}", msg)
            }
        }
    }
}

impl error::Error for Error {}

impl Text {
    pub fn from_path(path: String) -> Result<Self, Error> {
        let file = match File::open(&path) {
            Ok(ref mut f) => {
                let mut buf = String::with_capacity(4096);
                match f.read_to_string(&mut buf) {
                    Ok(_) => buf,
                    Err(e) => return Err(Error::OpenError(e.to_string()))
                }
            }
            Err(e) => return Err(Error::OpenError(e.to_string()))
        };

        let text: Vec<String> = file
            .lines()
            .map(Row::from)
            .collect();

        Ok(Self {
            path,
            text
        })
    }


}
