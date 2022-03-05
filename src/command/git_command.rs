pub struct GitCommand {
    options: Vec<String>,
    command: String,
    value: String
}

pub trait CommandBuilder {
    fn to_git_command() -> String;
}

impl CommandBuilder for GitCommand {
    fn to_git_command() -> String {
        String::from("")
    }
}