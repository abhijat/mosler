use errors::Error;
use serde_json::{Map, Value};
use utils::http_utils::empty;
use utils::json_utils::*;
use vault::http_client::VaultHTTPClient;

#[derive(Debug)]
pub struct VaultApi {
    pub http_client: VaultHTTPClient,
}

impl VaultApi {
    pub fn new(client: VaultHTTPClient) -> Self {
        VaultApi { http_client: client }
    }

    fn parse_json(r: Result<::reqwest::Response, Error>) -> Result<Value, String> {
        r.map_err(|e| e.to_string())?
            .json::<Value>()
            .map_err(|e| e.to_string())
    }

    pub fn probe(&self) -> Result<String, String> {
        Self::parse_json(self.http_client.get("sys/health"))
            .map(|v| format!("{:#}", v))
    }

    pub fn get_policies(&self) -> Result<String, String> {
        let value = Self::parse_json(self.http_client.get("sys/policy"))?;
        let value = get_object_from_path(&value, &vec!["data", "policies"])?;
        Ok(format!("{:#}", value))
    }

    pub fn get_app_roles(&self) -> Result<String, String> {
        let v = Self::parse_json(self.http_client.method("auth/approle/role", "LIST"))?;
        let v = get_string_from_path(&v, &vec!["data", "keys"])?;
        Ok(format!("{:#}", v))
    }

    pub fn read_policy(&self, policy_name: &str) -> Result<String, String> {
        let path = format!("sys/policy/{}", policy_name);
        let v = Self::parse_json(self.http_client.get(&path))?;

        let s = get_string_from_path(&v, &vec!["data", "rules"])?;
        Ok(format!("rules: {:#}", s))
    }

    pub fn enable_approle(&self) -> Result<String, String> {
        let path = format!("sys/auth/approle");
        let payload = json!({"type": "approle"});

        let mut response = self.http_client.post(&path, &payload)
            .map_err(|e| e.to_string())?;

        if empty(&response) {
            return Ok(format!("{}", response.status()));
        }

        let v = response.json::<Value>()
            .map_err(|e| e.to_string())?;

        let s = get_string_from_path(&v, &vec!["data", "keys"])?;
        Ok(format!("{:#}", s))
    }

    pub fn write_secret(&self, command: &[&str]) -> Result<String, String> {
        let secret_path = command[1];
        let secret_path = format!("secret/{}", secret_path);

        let mut payload = Map::new();

        command.iter().skip(2).for_each(|k| {
            eprintln!("k = {:?}", k);

            let v: Vec<&str> = k.split('=').collect();

            let key = v[0];
            let value: String = v.iter().skip(1).map(|s| *s).collect();

            payload.insert(key.to_owned(), json!(value));
        });

        let payload = json!(payload);

        eprintln!("payload = {:#}", payload);

        let mut response = self.http_client.post(&secret_path, &payload)
            .map_err(|e| e.to_string())?;

        if empty(&response) {
            return Ok(format!("{}", response.status()));
        }

        let v = response.json::<Value>()
            .map_err(|e| e.to_string())?;

        let s = get_string_from_path(&v, &vec!["data", "keys"])?;
        Ok(format!("{:#}", s))
    }
}
