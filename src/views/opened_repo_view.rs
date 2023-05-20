use crate::{
    command::command_handler::CommandHandler,
    commit_table::CommitTable,
    view_components::input_field::{self, InputField},
};

use crossterm::event::KeyCode;
use tui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::traits::display_view::DisplayView;

pub struct OpenedRepoView {
    pub repo_commits: CommitTable,
    pub input_field: InputField,
    pub force_draw: bool,
    pub handler: CommandHandler,
}

impl Default for OpenedRepoView {
    fn default() -> OpenedRepoView {
        let mut table = CommitTable {
            table_state: TableState::default(),
            table_items: vec![],
        };

        table.table_state.select(Some(0));
        OpenedRepoView {
            repo_commits: table,
            input_field: InputField::default(),
            force_draw: true,
            handler: CommandHandler::default(),
        }
    }
}

impl DisplayView for OpenedRepoView {
    fn display_view<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) -> bool {
        if !self.force_draw {
            if let Some(key_event) = self.input_field.input_wait() {
                if input_field::is_quit_event(&key_event) {
                    return false;
                } else if key_event.code == KeyCode::Down {
                    self.arrow_down();
                } else if key_event.code == KeyCode::Up {
                    self.arrow_up();
                } else if key_event.code == KeyCode::Enter {
                    self.input_field.enter_message();

                    let input = self
                        .input_field
                        .last_message()
                        .expect("Expected input after pushing message to message buffer");

                    if let Some(should_continue) = self.handler.call_handler(&input) {
                        return should_continue;
                    }
                }
            }
        } else {
            self.force_draw = false;
        }

        let rects = Layout::default()
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .margin(1)
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["Commit Message", "Author", "ID"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = self.repo_commits.table_items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(c.to_owned()));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]);

        f.render_stateful_widget(table, rects[0], &mut self.repo_commits.table_state);

        let input_field_text = Paragraph::new(self.input_field.input.value())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Run Command")
                    .style(Style::default().fg(Color::LightBlue)),
            )
            .style(Style::default().fg(Color::White));

        let input_y = rects[0].height + 2;
        let input_x = (self.input_field.input.cursor() + 2)
            .try_into()
            .unwrap_or(u16::max_value());
        f.set_cursor(input_x, input_y);

        f.render_widget(input_field_text, rects[1]);

        true
    }

    fn arrow_down(&mut self) {
        let i = match self.repo_commits.table_state.selected() {
            Some(i) => {
                let count = self.repo_commits.table_items.len();

                if count == 0 {
                    return;
                } else if i >= count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.repo_commits.table_state.select(Some(i));
    }

    fn arrow_up(&mut self) {
        let i = match self.repo_commits.table_state.selected() {
            Some(i) => {
                let count = self.repo_commits.table_items.len();

                if count == 0 {
                    return;
                } else if i == 0 {
                    count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.repo_commits.table_state.select(Some(i));
    }
}
