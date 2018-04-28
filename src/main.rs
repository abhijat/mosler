extern crate colored;
extern crate reqwest;
extern crate rustyline;
extern crate serde_json;

use vault::api::VaultApi;
use vault::http_client::VaultHTTPClient;
use vault::auth_context::ContextBuilder;
use foo::do_this;

mod shell;
mod command_handlers;
mod command_completer;
mod json_extractor;
mod vault;

fn main() {
    let ctx = ContextBuilder::new()
        .server_address("http://localhost:8200/v1")
        .auth_token("2db7ef2c-7449-35a7-7412-4a1018c82a7a")
        .build();

    let client = VaultHTTPClient::new(ctx);
    let api = VaultApi::new(client);
    shell::shell(api);
}
