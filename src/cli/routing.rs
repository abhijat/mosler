use vault::api::VaultApi;

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
