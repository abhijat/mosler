use reqwest::Response;
use reqwest::header::ContentLength;

pub fn size(r: &Response) -> u64 {
    r.headers().get::<ContentLength>()
        .map(|x| **x)
        .unwrap_or(0)
}

pub fn empty(r: &Response) -> bool {
    size(r) == 0
}