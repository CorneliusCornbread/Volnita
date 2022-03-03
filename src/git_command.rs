pub struct GitCommand {
    options: Vec<String>,
    command: String,
    value: String
}

pub enum Command {
    push
}

impl GitCommand {
    pub fn to_git_command() -> String {
        String::from("")
    }
}