use auth_context::AuthContext;
use reqwest::{Client, Error, Response, StatusCode, Url};
use reqwest::header::{ContentType, Headers};
use reqwest::Method::Extension;

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
        // TODO strip off extra slashes here
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