extern crate rustyline;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod shell;
mod command_parser;
mod command_completer;

fn main() {
    shell::shell();
}
