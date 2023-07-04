use std::{collections::HashMap, str::Split};

use super::construct_handler::handler_create_callbacks;

pub struct CommandHandler {
    /// Handler returns a boolean, false meaning to quit the application, true meaning to continue execution
    pub(in crate::command) handlers: HashMap<String, fn(Split<char>) -> bool>,
}

impl Default for CommandHandler {
    fn default() -> Self {
        handler_create_callbacks()
    }
}

impl CommandHandler {
    pub fn add_handler(&mut self, command: &str, callback: fn(Split<char>) -> bool) {
        self.handlers.insert(command.to_owned(), callback);
    }

    pub fn call_handler(&self, command: &str) -> Option<bool> {
        let mut split = command.split(' ');

        self.handlers
            .get(
                split.next().expect(
                    "call_handler() split contains no data in the first entry when it should",
                ),
            )
            .map(|callback| callback(split))
    }
}
