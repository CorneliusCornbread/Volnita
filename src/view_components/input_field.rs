use tui_input::Input;

use crate::input_mode::InputMode;

pub struct InputField {
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>
}