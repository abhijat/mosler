use rustyline::completion::Completer;
use rustyline::error::ReadlineError;

pub struct CommandCompleter {}

impl CommandCompleter {
    pub fn commands() -> Vec<String> {
        vec![
            "ls-policies".to_owned(),
            "ls-approles".to_owned(),
            "read-policy".to_owned(),
            "enable-approle".to_owned(),
            "write-secret".to_owned(),
        ]
    }
}

impl Completer for CommandCompleter {
    // In this function we handle command completion. We check if the user input matches
    // one of our pre-defined commands. Here 'matches' means the command starts with the
    // input. This is mostly brute force and not very elegant for now.
    fn complete(&self, line: &str, _pos: usize) -> Result<(usize, Vec<String>), ReadlineError> {
        let commands = &CommandCompleter::commands();

        // First we filter out those commands which actually start with the input.
        let candidates: Vec<String> = commands.iter()
            .filter(|c| c.starts_with(line))
            .map(|c| c.clone())
            .collect();

        if candidates.is_empty() {
            Ok((0, vec![]))
        } else {
            Ok((0, candidates.clone()))
        }
    }
}
