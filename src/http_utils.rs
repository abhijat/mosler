use auth_context::AuthContext;
use auth_context::ContextBuilder;
use reqwest::{Client, Error, Response, StatusCode, Url};
use reqwest::header::{ContentType, Headers};
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

#[derive(Debug)]
pub struct VaultHTTPClient {
    auth_context: AuthContext
}

impl VaultHTTPClient {
    pub fn new(ctx: AuthContext) -> Self {
        VaultHTTPClient {
            auth_context: ctx,
        }
    }

    fn normalize(&self, p: &str) -> String {
        format!("{}/{}", self.auth_context.server_address, p)
    }

    fn auth_header(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set_raw("X-Vault-Token", self.auth_context.auth_token.clone());
        headers.set(ContentType::json());
        headers
    }

    pub fn get(&self, path: &str) -> Result<Response, Error> {
        let client = Client::new();
        let response = client.get(&self.normalize(path))
            .headers(self.auth_header())
            .send()
            .unwrap();

        self.map_response(response)
    }

    pub fn method(&self, path: &str, method_type: &str) -> Result<Response, Error> {
        let method = Extension(method_type.to_string());
        let url = Url::parse(&self.normalize(path)).unwrap();

        let response = Client::new()
            .request(method, url)
            .headers(self.auth_header())
            .send()
            .unwrap();

        self.map_response(response)
    }

    fn map_response(&self, r: Response) -> Result<Response, Error> {
        match r.status() {
            StatusCode::Ok => Ok(r),
            _ => r.error_for_status(),
        }
    }
}