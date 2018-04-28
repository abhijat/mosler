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

    // TODO build up context from cmdline input
    // TODO research terminal input libs

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        println!("usage: mosler <token>");
        std::process::exit(1);
    }

    let token = &args[0];

    let ctx = ContextBuilder::new()
        .server_address("http://localhost:8200/v1")
        .auth_token(token)
        .build();

    let client = VaultHTTPClient::new(ctx);
    let api = VaultApi::new(client);
    shell::shell(api);
}
