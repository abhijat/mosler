use reqwest::header::{Headers, ContentType};
use reqwest::Response;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Error;

static SERVER_URL: &str = "http://localhost:8200/v1";

pub fn auth_header(token: &str) -> Headers {
    let mut headers = Headers::new();
    headers.set_raw("X-Vault-Token", token.to_owned());
    headers.set(ContentType::json());
    headers
}

pub fn http_get(path: &str, token: &str) -> Result<Response, Error> {
    let client = Client::new();
    let path = format!("{}/{}", SERVER_URL, path);

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