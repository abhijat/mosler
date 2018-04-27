use std::collections::HashMap;
use reqwest::header::Headers;
use reqwest::Client;
use reqwest::header::ContentType;

#[derive(Debug, Deserialize)]
struct Policies {
    data: HashMap<String, Vec<String>>
}

pub fn get_policies() -> String {
    let client = Client::new();

    let mut headers = Headers::new();

    headers.set_raw("X-Vault-Token", "8acd0b34-a220-bfd3-de8d-7fc2ab05c22c");
    headers.set(ContentType::json());

    let mut response = client.get("http://localhost:8200/v1/sys/policy")
        .headers(headers)
        .send()
        .unwrap();

    let response: Policies = response.json().unwrap();
    format!("{:?}", response.data.get("policies").unwrap())
}

pub fn parse_command(command: &str) -> String {
    if command == "ls-policies" {
        get_policies()
    } else {
        "unknown command!".to_owned()
    }
}
