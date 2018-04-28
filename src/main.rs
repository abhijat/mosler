extern crate colored;
extern crate reqwest;
extern crate rustyline;
extern crate serde_json;

use auth_context::ContextBuilder;
use vault_api::VaultApi;
use http_utils::VaultHTTPClient;

mod shell;
mod command_parser;
mod command_completer;
mod http_utils;
mod json_utils;
mod auth_context;
mod vault_api;

fn main() {

    let ctx = ContextBuilder::new()
        .server_address("http://localhost:8200/v1")
        .auth_token("2db7ef2c-7449-35a7-7412-4a1018c82a7a")
        .build();

    let client = VaultHTTPClient::new(ctx);
    let api = VaultApi::new(client);
    shell::shell(api);
}
