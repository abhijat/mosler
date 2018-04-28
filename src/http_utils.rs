use reqwest::header::{Headers, ContentType};
use reqwest::{Response, Client, StatusCode, Error, Url};
use reqwest::Method::Extension;

static SERVER_URL: &str = "http://localhost:8200/v1";

pub fn auth_header(token: &str) -> Headers {
    let mut headers = Headers::new();
    headers.set_raw("X-Vault-Token", token.to_owned());
    headers.set(ContentType::json());
    headers
}

pub fn http_get(path: &str, token: &str) -> Result<Response, Error> {
    let path = format!("{}/{}", SERVER_URL, path);

    let client = Client::new();
    let response = client.get(&path)
        .headers(auth_header(token))
        .send()
        .unwrap();

    match response.status() {
        StatusCode::Ok => {
            Ok(response)
        }

        _ => {
            response.error_for_status()
        }
    }
}

pub fn http_custom(path: &str, token: &str, method_type: &str) -> Result<Response, Error> {

    let path = format!("{}/{}", SERVER_URL, path);

    let method = Extension(method_type.to_string());
    let url = Url::parse(&path).unwrap();

    let response = Client::new()
        .request(method, url)
        .headers(auth_header(token))
        .send()
        .unwrap();

    match response.status() {
        StatusCode::Ok => {
            Ok(response)
        }

        _ => {
            response.error_for_status()
        }
    }
}