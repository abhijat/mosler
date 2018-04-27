use rustyline::completion::Completer;
use rustyline::completion::longest_common_prefix;
use rustyline::error::ReadlineError;

pub struct CommandCompleter {}

impl CommandCompleter {
    pub fn commands() -> Vec<String> {
        vec!["ls-policies".to_owned(), "ls-approles".to_owned()]
    }
}

impl Completer for CommandCompleter {

    // In this function we handle command completion. We check if the user input matches
    // one of our pre-defined commands. Here 'matches' means the command starts with the
    // input. This is mostly brute force and not very elegant for now.
    fn complete(&self, line: &str, pos: usize) -> Result<(usize, Vec<String>), ReadlineError> {

        let commands = &CommandCompleter::commands();

        // Some common prefix of our commands starts with input, show those commands
        // as completion candidates.
        if let Some(prefix) = longest_common_prefix(commands) {
            if prefix.starts_with(line) {
                return Ok((0, commands.clone()));
            }
        }

        // No common prefix starts with the input, See if any actual command starts with
        // the user input. If so return just that one command as candidate.
        for command in commands {
            if command.starts_with(line) {
                return Ok((0, vec![command.to_owned()]));
            }
        }

        // Matched nothing
        Ok((0, vec![]))

    }
}
