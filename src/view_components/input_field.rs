use tui_input::Input;

use crate::{input_mode::InputMode};

pub struct InputField {
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>
}

impl InputField {
    pub fn enter_message(&mut self) {
        self.messages.push(self.input.value().to_owned());
        self.input.reset();
    }
}

impl Default for InputField {
    fn default() -> Self {
        InputField { input: Input::default(), input_mode: InputMode::Editing, messages: Vec::new() }
        
    }
}
