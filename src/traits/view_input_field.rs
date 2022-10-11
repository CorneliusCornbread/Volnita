use crate::input_mode::InputMode;

pub trait ViewInputField {
    fn get_input_mode(&self) -> InputMode;

    fn set_input_mode(&mut self, mode: InputMode);
    
    fn enter_message(&mut self, message: &str);
}
