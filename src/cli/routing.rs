use vault::api::VaultApi;
use term_painter::Painted;
use cli::colors::paint_error;

fn route_command_with_args(api: &VaultApi, command: &str) -> Painted<String> {
    let tokens: Vec<&str> = command.split_whitespace().collect();

    if tokens.len() < 2 {
        return paint_error("invalid command".to_owned());
    }

    match tokens[0] {
        "read-policy" => api.read_policy(tokens[1]),
        _ => paint_error("unknown command".to_owned())
    }
}

pub fn command_router(api: &VaultApi, command: &str) -> Painted<String> {
    match command {
        "ls-policies" => api.get_policies(),
        "ls-approles" => api.get_app_roles(),
        "enable-approle" => api.enable_approle(),
        s if s.contains(" ") => {
            route_command_with_args(&api, s)
        }
        _ => paint_error("unknown command!".to_owned())
    }
}
