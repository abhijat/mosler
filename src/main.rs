extern crate rustyline;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate colored;
extern crate serde_json;

mod shell;
mod command_parser;
mod command_completer;
mod http_utils;
mod json_utils;

fn main() {
    shell::shell();
}
