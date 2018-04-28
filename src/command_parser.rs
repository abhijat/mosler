use colored::*;
use http_utils::{http_custom, http_get};
use json_utils::JsonExtractor;
use reqwest::Error;
use serde_json::Value;
use vault_api::VaultApi;

static ROOT_TOKEN: &str = "2db7ef2c-7449-35a7-7412-4a1018c82a7a";

fn format_error(e: Error) -> String {
    e.to_string()
        .red()
        .bold()
        .to_string()
}

pub fn get_policies() -> String {
    let response = http_get("sys/policy", ROOT_TOKEN);

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

pub fn get_app_roles() -> String {
    let response = http_custom("auth/approle/role", ROOT_TOKEN, "LIST");

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

pub fn read_policy(policy_name: &str) -> String {
    let path = format!("sys/policy/{}", policy_name);
    let response = http_get(&path, ROOT_TOKEN);

    match response {
        Err(e) => {
            format_error(e)
        }

        Ok(mut r) => {
            let v: Value = r.json().unwrap();
            format!("rules: {:#}", JsonExtractor::new(&v)
                .get_value("data")
                .get_str("rules"))
        }
    }
}

fn route_command_with_args(api: &VaultApi, command: &str) -> String {
    let tokens: Vec<&str> = command.split_whitespace().collect();

    if tokens.len() < 2 {
        return "invalid command".to_owned();
    }

    match tokens[0] {
        "read-policy" => api.read_policy(tokens[1]),
        _ => "unknown command".to_owned()
    }
}

pub fn command_router(api: &VaultApi, command: &str) -> String {
    match command {
        "ls-policies" => api.get_policies(),
        "ls-approles" => api.get_app_roles(),
        s if s.contains(" ") => {
            route_command_with_args(&api, s)
        }
        _ => "unknown command!".to_owned()
    }
}
