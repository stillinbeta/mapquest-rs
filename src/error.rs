extern crate reqwest;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::ReqwestError(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::SerdeError(source)
    }
}
