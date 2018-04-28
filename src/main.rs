extern crate colored;
extern crate reqwest;
extern crate rustyline;
extern crate serde_json;
extern crate term_painter;

use vault::api::VaultApi;
use vault::http_client::VaultHTTPClient;
use vault::auth_context::ContextBuilder;

mod shell;
mod json_extractor;
mod vault;
mod cli;

fn main() {
    let ctx = ContextBuilder::new()
        .server_address("http://localhost:8200/v1")
        .auth_token("2fdffa86-fb08-a54e-bdfd-3f5bfc63bc92")
        .build();

    let client = VaultHTTPClient::new(ctx);
    let api = VaultApi::new(client);
    shell::shell(api);
}
