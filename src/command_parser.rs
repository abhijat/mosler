use colored::*;
use http_utils::http_custom;
use http_utils::http_get;
use json_utils::key_from_object;
use json_utils::str_from_object;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Policies {
    data: HashMap<String, Vec<String>>
}

#[derive(Debug, Deserialize)]
struct AppRoles {
    data: HashMap<String, Vec<String>>
}

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
            let policies: Policies = r.json().unwrap();
            format!("{:?}", policies.data.get("policies").unwrap())
        }

        Err(e) => {
            format_error(e)
        }
    }
}

pub fn get_approles() -> String {
    let response = http_custom("auth/approle/role", ROOT_TOKEN, "LIST");

    match response {
        Ok(mut r) => {
            let approles: AppRoles = r.json().unwrap();
            format!("{:?}", approles.data.get("keys").unwrap())
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
        Ok(mut r) => {
            let v: Value = r.json().unwrap();
            let data = key_from_object(&v, "data");
            format!(
                "name: {:#} rules: {:#}",
                str_from_object(data, "name"),
                str_from_object(data, "rules")
            )
        }

        Err(e) => {
            format_error(e)
        }
    }
}

pub fn parse_command(command: &str) -> String {
    match command {
        "ls-policies" => get_policies(),
        "ls-approles" => get_approles(),
        s if s.starts_with("read-policy") => {
            let tokens: Vec<&str> = s.split_whitespace().collect();

            if tokens.len() != 2 {
                "invalid command".to_owned()
            } else {
                read_policy(tokens.get(1).unwrap())
            }
        }
        _ => "unknown command!".to_owned()
    }
}
