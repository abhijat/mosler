use std::fmt;
use ::reqwest::{Response, StatusCode};

#[derive(Debug)]
pub enum ErrorKind {
    ReqwestError(::reqwest::Error),
    UrlParseError(::reqwest::UrlError),
}

impl ErrorKind {
    pub fn map_http_code(r: Response) -> Result<Response, ErrorKind> {
        match r.status() {
            StatusCode::Ok => Ok(r),
            _ => r.error_for_status()
                .map_err(|e| ErrorKind::ReqwestError(e)),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ErrorKind::ReqwestError(ref e) => write!(f, "http request error: {:?}", e),
            ErrorKind::UrlParseError(ref e) => write!(f, "url parser error: {:?}", e),
        }
    }
}
