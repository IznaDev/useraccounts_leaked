use std::error;
use std::fmt;

//enum which brings together the different types of error
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NoColon,
    ReqwestError(reqwest::Error),
    ParseIntError(std::num::ParseIntError),
}

//FOr each error type I had to implement The traits as below to avoid errors of compilation
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error !!!!")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        let error_mess = "err";
        error_mess
    }
}
