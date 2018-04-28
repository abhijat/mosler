use std::collections::HashMap;
use http_utils::http_get;

use colored::*;
use reqwest::Error;

#[derive(Debug, Deserialize)]
struct Policies {
    data: HashMap<String, Vec<String>>
}

#[derive(Debug, Deserialize)]
struct AppRoles {
    data: HashMap<String, Vec<String>>
}

static ROOT_TOKEN: &str = "648325b0-7de4-59d3-8541-9fc103bd5b0e";

fn format_error(e: Error) -> String {
    e.to_string()
        .red()
        .bold()
        .to_string()
}

pub fn get_policies() -> String {
    let response = http_get("/sys/policy", ROOT_TOKEN);

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
    let response = http_get("/auth/approle/role", ROOT_TOKEN);

    match response {
        Ok(mut r) => {
            let approles: AppRoles = r.json().unwrap();
            format!("{:?}", approles)
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
        _ => "unknown command!".to_owned()
    }
}
