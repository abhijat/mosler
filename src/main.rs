extern crate colored;
extern crate reqwest;
extern crate rustyline;
extern crate serde_json;

mod shell;
mod command_parser;
mod command_completer;
mod http_utils;
mod json_utils;

fn main() {
    shell::shell();
}
