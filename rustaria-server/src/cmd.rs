pub struct Commands {
    // TODO(leocth): do we want some HashMap here to store the command handlers or sth?
}

impl Commands {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(cmd: &str) -> Option<Command> {
        // TODO(leocth): replace with actual logic
        match cmd {
            "reload" => Some(Command::Reload),
            _ => None,
        }
    }
}

// TODO: this is definitely not going to be an enum, as this should be user-customizable
#[derive(Debug)]
pub enum Command {
    Reload,
}
