use json_extractor::JsonExtractor;
use serde_json::Value;
use vault::http_client::VaultHTTPClient;
use term_painter::Painted;
use cli::colors::paint_error;
use cli::colors::paint_success;

#[derive(Debug)]
pub struct VaultApi {
    pub http_client: VaultHTTPClient,
}

impl VaultApi {
    pub fn new(client: VaultHTTPClient) -> Self {
        VaultApi { http_client: client }
    }

    pub fn probe(&self) -> Result<String, String> {
        match self.http_client.get("sys/health") {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                Ok(format!("{:#}", v))
            }
            Err(e) => {
                Err(e.to_string())
            }
        }

    }

    pub fn get_policies(&self) -> Painted<String> {
        let response = self.http_client.get("sys/policy");

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                paint_success(format!("{:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_value("policies")
                    .value_ref))
            }

            Err(e) => {
                paint_error(e.to_string())
            }
        }
    }

    pub fn get_app_roles(&self) -> Painted<String> {
        let response = self.http_client.method("auth/approle/role", "LIST");

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                paint_success(format!("{:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_str("keys")))
            }

            Err(e) => {
                paint_error(e.to_string())
            }
        }
    }

    pub fn read_policy(&self, policy_name: &str) -> Painted<String> {
        let path = format!("sys/policy/{}", policy_name);
        let response = self.http_client.get(&path);

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                paint_success(format!("rules: {:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_str("rules")))
            }

            Err(e) => {
                paint_error(e.to_string())
            }
        }
    }
}
