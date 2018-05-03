use errors::Error;
use reqwest::{Client, Response, Url};
use reqwest::header::{ContentType, Headers};
use reqwest::Method::Extension;
use vault::auth_context::AuthContext;

#[derive(Debug)]
pub struct VaultHTTPClient {
    pub auth_context: AuthContext
}

impl VaultHTTPClient {
    pub fn new(ctx: AuthContext) -> Self {
        VaultHTTPClient {
            auth_context: ctx,
        }
    }

    fn normalize(&self, p: &str) -> String {
        if p.starts_with("/") {
            format!("{}{}", self.auth_context.server_address, p)
        } else {
            format!("{}/{}", self.auth_context.server_address, p)
        }
    }

    fn auth_header(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set_raw("X-Vault-Token", self.auth_context.auth_token.clone());
        headers.set(ContentType::json());
        headers
    }

    pub fn get(&self, path: &str) -> Result<Response, Error> {
        Client::new()
            .get(&self.normalize(path))
            .headers(self.auth_header())
            .send()
            .map_err(Error::HttpRequestError)
            .and_then(Error::map_http_code)
    }

    pub fn method(&self, path: &str, method_type: &str) -> Result<Response, Error> {
        let method = Extension(method_type.to_string());
        let path = self.normalize(path);

        let url = Url::parse(&path)?;
        let response = Client::new()
            .request(method, url)
            .headers(self.auth_header())
            .send()?;

        Error::map_http_code(response)
    }
}