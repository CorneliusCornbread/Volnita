use crossterm::event::{KeyCode, Event, self};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::input_mode::InputMode;

pub struct InputField {
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>
}

impl InputField {
    //TODO: just rewrite this whole function, it's bad
    pub fn check_input(&mut self) -> Option<KeyCode> {
        if let Event::Key(key) = event::read().expect("Error reading") {
            match key.code {
                //KeyCode::Char('q') => return Ok(()), //TODO: need to implement quitting
                KeyCode::Down => return Some(KeyCode::Down),
                KeyCode::Up => return Some(KeyCode::Up),
                _ => {}
            }

            //TODO: make this less bad
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        self.input_mode = InputMode::Editing;
                        return Some(KeyCode::Char('e'))
                    }
                    KeyCode::Char('q') => {
                        return Some(KeyCode::Char('q'));
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        self.enter_message();
                        
                    }
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                    }
                    _ => {
                        self.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }

        Some(KeyCode::Null)
    }

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
