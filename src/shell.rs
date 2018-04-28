use ::command_parser::parse_command;
use command_completer::CommandCompleter;
use rustyline::config::{Builder, CompletionType};
use rustyline::error::ReadlineError;

pub fn shell() {
    let builder = Builder::new().completion_type(CompletionType::List);
    let config = builder.build();

    let mut rl = ::rustyline::Editor::<CommandCompleter>::with_config(config);

    rl.set_completer(Some(CommandCompleter {}));

    if let Err(_) = rl.load_history(".mosler_history") {
        println!("no history to load");
    }

    loop {
        let input = rl.readline("[the shell!] $ ");
        match input {
            Ok(s) => {
                if !s.is_empty() {
                    rl.add_history_entry(s.clone());
                    let response = parse_command(&s);
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

