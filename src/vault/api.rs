use json_extractor::JsonExtractor;
use serde_json::Value;
use utils::http_utils::empty;
use vault::http_client::VaultHTTPClient;

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

    pub fn get_policies(&self) -> Result<String, String> {
        self.http_client.get("sys/policy")
            .map_err(|e| e.to_string())?
            .json::<Value>()
            .map_err(|e| e.to_string())
            .map(|v| format!("{:#}", JsonExtractor::new(&v)
                .get_value("data")
                .get_value("policies")
                .value_ref))
    }

    pub fn get_app_roles(&self) -> Result<String, String> {
        self.http_client.method("auth/approle/role", "LIST")
            .map_err(|e| e.to_string())?
            .json::<Value>()
            .map_err(|e| e.to_string())
            .map(|v| format!("{:#}", JsonExtractor::new(&v)
                .get_value("data")
                .get_str("keys")))
    }

    pub fn read_policy(&self, policy_name: &str) -> Result<String, String> {
        let path = format!("sys/policy/{}", policy_name);
        self.http_client.get(&path)
            .map_err(|e| e.to_string())?
            .json::<Value>()
            .map_err(|e| e.to_string())
            .map(|v| format!("rules: {:#}", JsonExtractor::new(&v)
                .get_value("data")
                .get_str("rules")))
    }

    pub fn enable_approle(&self) -> Result<String, String> {
        let path = format!("sys/auth/approle");
        let payload = json!({"type": "approle"});

        let mut response = self.http_client.post(&path, &payload)
            .map_err(|e| e.to_string())?;

        if empty(&response) {
            return Ok(format!("{}", response.status()));
        }

        response.json::<Value>()
            .map_err(|e| e.to_string())
            .map(|v| format!("{:#}", JsonExtractor::new(&v)
                .get_value("data")
                .get_str("keys")))
    }
}
