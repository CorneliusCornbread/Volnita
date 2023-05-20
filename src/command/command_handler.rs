use std::collections::HashMap;

use super::construct_handler::handler_create_callbacks;

pub struct CommandHandler {
    pub(in crate::command) handlers: HashMap<String, fn(Vec<&str>) -> bool>,
}

impl Default for CommandHandler {
    fn default() -> Self {
        handler_create_callbacks()
    }
}

impl CommandHandler {
    pub fn add_handler(&mut self, command: &str, callback: fn(Vec<&str>) -> bool) {
        self.handlers.insert(command.to_owned(), callback);
    }

    pub fn call_handler(&self, command: &str) -> Option<bool> {
        let split: Vec<&str> = command.split(" ").collect();
        if let Some(callback) = self.handlers.get(split[0]) {
            return Some(callback(split));
        } else {
            None
        }
    }
}
