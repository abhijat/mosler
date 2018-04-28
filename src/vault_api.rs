use colored::*;
use vault_http_client::VaultHTTPClient;
use json_extractor::JsonExtractor;
use reqwest::Error;
use serde_json::Value;

#[derive(Debug)]
pub struct VaultApi {
    pub http_client: VaultHTTPClient,
}

impl VaultApi {
    pub fn new(client: VaultHTTPClient) -> Self {
        VaultApi { http_client: client }
    }

    pub fn get_policies(&self) -> String {
        let response = self.http_client.get("sys/policy");

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                format!("{:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_value("policies")
                    .value_ref)
            }

            Err(e) => {
                format_error(e)
            }
        }
    }

    pub fn get_app_roles(&self) -> String {
        let response = self.http_client.method("auth/approle/role", "LIST");

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                format!("{:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_str("keys"))
            }

            Err(e) => {
                format_error(e)
            }
        }
    }

    pub fn read_policy(&self, policy_name: &str) -> String {
        let path = format!("sys/policy/{}", policy_name);
        let response = self.http_client.get(&path);

        match response {
            Ok(mut r) => {
                let v: Value = r.json().unwrap();
                format!("rules: {:#}", JsonExtractor::new(&v)
                    .get_value("data")
                    .get_str("rules"))
            }

            Err(e) => {
                format_error(e)
            }
        }
    }
}

fn format_error(e: Error) -> String {
    e.to_string().red().bold().to_string()
}
