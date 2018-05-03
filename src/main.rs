extern crate colored;
extern crate reqwest;
extern crate rpassword;
extern crate rustyline;

#[macro_use]
extern crate serde_json;
extern crate term_painter;

use cli::colors::{paint, paint_error};
use std::io::{self, Write};
use vault::api::VaultApi;
use vault::auth_context::AuthContext;
use vault::auth_context::ContextBuilder;
use vault::http_client::VaultHTTPClient;

mod shell;
mod json_extractor;
mod vault;
mod cli;
mod errors;

fn read_input() -> AuthContext {
    let mut server = String::new();
    print!("Please enter the url to the vault server [http://localhost:8200/v1] : ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut server).expect("failed to read vault server url");

    if server.trim().is_empty() {
        server = "http://localhost:8200/v1".to_owned();
    }

    let token = rpassword::prompt_password_stdout("Please enter your vault auth token: ")
        .expect("failed to read token from command line!");

    ContextBuilder::new()
        .server_address(server.trim())
        .auth_token(&token)
        .build()
}

fn main() {
    let ctx = read_input();
    let client = VaultHTTPClient::new(ctx);
    let api = VaultApi::new(client);

    println!("probing server {}", api.http_client
        .auth_context
        .server_address);

    let response = api.probe();
    let failed = response.is_err();

    println!("{}", paint(response));

    if failed {
        println!("{}", paint_error("failed to probe server. exiting!".to_owned()));
        std::process::exit(1);
    }

    shell::shell(api);
}
