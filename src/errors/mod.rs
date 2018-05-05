use std::fmt;
use std::error::Error as StdError;
use ::reqwest::{Response, StatusCode, UrlError, Error as ReqwestError};

#[derive(Debug)]
pub enum Error {
    HttpRequestError(ReqwestError),
    UrlParseError(UrlError),
    JsonParseError,
    KeyNotFound(String),
    JsonToStringFailed,
}

impl Error {
    pub fn map_http_code(r: Response) -> Result<Response, Error> {
        match r.status() {
            StatusCode::Ok => Ok(r),
            _ => r.error_for_status().map_err(Error::HttpRequestError)
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpRequestError(ref e) => e.description(),
            Error::UrlParseError(_) => "failed to parse the url",
            Error::JsonParseError => "failed to parse json value",
            Error::KeyNotFound(ref s) => &s,
            Error::JsonToStringFailed => "failed to convert json to string",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::HttpRequestError(ref e) => e.cause(),
            Error::UrlParseError(_) => None,
            Error::JsonParseError => None,
            Error::KeyNotFound(_) => None,
            Error::JsonToStringFailed => None,
        }
    }
}

impl From<::reqwest::Error> for Error {
    fn from(e: ReqwestError) -> Self {
        Error::HttpRequestError(e)
    }
}

impl From<::reqwest::UrlError> for Error {
    fn from(e: UrlError) -> Self {
        Error::UrlParseError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::HttpRequestError(ref e) => write!(f, "{}", e),
            Error::UrlParseError(ref e) => write!(f, "{}", e),
            Error::JsonParseError => write!(f, "failed to parse object from json value"),
            Error::KeyNotFound(ref s) => write!(f, "could not find key {} in json", s),
            Error::JsonToStringFailed => write!(f, "could not convert json to string"),
        }
    }
}
