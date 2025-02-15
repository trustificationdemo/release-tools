pub enum ActionCommand {
    Debug(String),
    Notice(String),
    Warning(String),
    Error(String),
}

impl ActionCommand {
    pub fn send_command(&self) {
        let (command, message) = match self {
            Self::Debug(message) => ("debug", message),
            Self::Notice(message) => ("notice", message),
            Self::Warning(message) => ("warning", message),
            Self::Error(message) => ("error", message),
        };

        println!("::{}::{}", command, message);
    }
}
