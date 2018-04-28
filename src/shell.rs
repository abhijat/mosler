use ::command_parser::command_router;
use command_completer::CommandCompleter;
use rustyline::config::{Builder, CompletionType};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use vault_api::VaultApi;

fn init_readline() -> Editor<CommandCompleter> {
    let builder = Builder::new()
        .completion_type(CompletionType::List);
    let config = builder.build();

    let mut rl = Editor::<CommandCompleter>::with_config(config);
    rl.set_completer(Some(CommandCompleter {}));
    if let Err(_) = rl.load_history(".mosler_history") {
        println!("no history to load");
    }

    rl
}

pub fn shell(api: VaultApi) {

    let mut rl = init_readline();

    loop {
        let input = rl.readline("[mosler] $ ");
        match input {
            Ok(s) => {
                if !s.is_empty() {
                    rl.add_history_entry(s.clone());
                    let response = command_router(&api, &s);
                    println!("{}", response);
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("aborted");
            }

            Err(ReadlineError::Eof) => {
                println!("good bye");
                break;
            }

            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(".mosler_history").unwrap();
}

