use crossterm::event::{KeyCode, Event, self};
use tui::{widgets::{Block, Borders}, Terminal, backend::Backend};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::input_mode::InputMode;

pub struct InputField {
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String> //TODO: change this back to &str vec
}

impl InputField {
    pub fn check_input(&mut self) -> Option<KeyCode> {
        if let Ok(Event::Key(key)) = event::read() {
            self.input.handle_event(&Event::Key(key));
            return Some(key.code);
        }

        None
    }

    pub fn input_prompt<B: Backend>(&mut self, terminal: &mut Terminal<B>, msg: &str) -> std::io::Result<&str> {
        self.input_mode = InputMode::Editing;

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .title(msg.to_owned() + "\n" + self.input.value())
                    .borders(Borders::NONE);
                f.render_widget(block, size);
            })?;
            
            if let Some(char) = self.check_input() {
                if char == KeyCode::Enter {
                    self.messages.push(self.input.value().to_owned());
                    return Ok(self.messages.last().unwrap());
                }
            }
        }
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
