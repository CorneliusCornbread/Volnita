use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    Terminal,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::input_mode::InputMode;

pub struct InputField {
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}

impl InputField {
    pub fn input_wait(&mut self) -> Option<KeyEvent> {
        if let Ok(Event::Key(key)) = event::read() {
            self.input.handle_event(&Event::Key(key));
            return Some(key);
        }

        None
    }

    pub fn input_prompt<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        msg: &str,
    ) -> std::io::Result<&str> {
        self.input_mode = InputMode::Editing;

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .title(msg.to_owned() + "\n" + self.input.value())
                    .borders(Borders::NONE);
                let cursor_x = msg.len() + self.input.cursor();
                f.set_cursor(cursor_x.try_into().unwrap_or(u16::max_value()), 0);
                f.render_widget(block, size);
            })?;

            if let Some(key_event) = self.input_wait() {
                if is_quit_event(&key_event) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Quitting application",
                    ));
                } else if key_event.code == KeyCode::Enter {
                    self.enter_message();
                    return Ok(self
                        .messages
                        .last()
                        .expect("Expected input prompt to have message after pushing a value."));
                }
            }
        }
    }

    pub fn enter_message(&mut self) {
        self.messages.push(self.input.value().to_owned());
        self.input.reset();
    }

    pub fn last_message(&self) -> Option<String> {
        self.messages.last().cloned()
    }
}

impl Default for InputField {
    fn default() -> Self {
        InputField {
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
        }
    }
}

pub fn is_quit_event(key_event: &KeyEvent) -> bool {
    key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c')
}
