use crate::{commit_table::CommitTable, view_components::input_field::InputField};

use crossterm::event::KeyCode;
use tui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

use crate::traits::display_view::DisplayView;

pub struct OpenedRepoView {
    pub repo_commits: CommitTable,
    pub input_field: InputField,
    pub force_draw: bool,
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
        }
    }
}

impl DisplayView for OpenedRepoView {
    fn display_view<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) -> bool {
        if !self.force_draw {
            if let Some(code) = self.input_field.check_input(){
                if code == KeyCode::Down {
                    self.arrow_down();
                } else if code == KeyCode::Up {
                    self.arrow_up();
                } else if code == KeyCode::Char('q') {
                    return false;
                }
            }
        }
        else {
            self.force_draw = false;
        }

        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["Commit Message", "Author", "ID"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
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
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]);

        f.render_stateful_widget(t, rects[0], &mut self.repo_commits.table_state);

        // TODO: this should be done in a nicer, more abstracted way.
        // We need some way of running a draw loop of the application once without
        // blocking the draws, maybe move input onto a separate thread entirely?
        // Otherwise we need to figure out some flag within our views.
        if self.force_draw {
            self.force_draw = false;
            return true
        }

        return true
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
