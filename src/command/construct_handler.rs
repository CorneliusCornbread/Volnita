use std::collections::HashMap;

use super::command_handler::CommandHandler;

pub fn handler_create_callbacks() -> CommandHandler {
    let mut handler = CommandHandler {
        handlers: HashMap::default(),
    };

    handler.add_handler("quit", |_h| false);

    handler
}
