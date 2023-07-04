use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use git2::string_array::StringArray;
use tui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::{
    command::command_handler::CommandHandler,
    config::{
        repo::{Repository, SavedRepositories},
        Config,
    },
    data_table::DataTable,
    traits::display_view::DisplayView,
    view_components::input_field::{self, InputField},
};

pub struct StartView {
    pub repositories: DataTable,
    pub input_field: InputField,
    force_draw: bool,
    pub handler: CommandHandler,
    pub repo_selected: Option<Repository>,
}

impl StartView {
    pub fn load_table(&mut self) {
        self.repositories.table_items = Self::get_table_items();
    }

    fn get_table_items() -> Vec<Vec<String>> {
        let config = SavedRepositories::load_or_create_config();

        let mut table_items = vec![vec![]];
        for repo in config.recent_repositories {
            table_items.push(vec![repo.name, repo.path, repo.repo_url])
        }

        table_items
    }
}

impl Default for StartView {
    fn default() -> Self {
        Self {
            repositories: DataTable {
                table_state: TableState::default(),
                table_items: StartView::get_table_items(),
            },
            input_field: InputField::default(),
            force_draw: true,
            handler: CommandHandler::default(),
            repo_selected: None,
        }
    }
}

impl DisplayView for StartView {
    fn display_view<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) -> bool {
        if !self.force_draw {
            if let Some(key_event) = self.input_field.input_wait() {
                if input_field::is_quit_event(&key_event) {
                    return false;
                }

                // Technically this is only set on Windows by default as we're not using the flags for
                // this flag to be set on Windows. Without the flags for Unix OS's it will always
                // default to 0, aka KeyEventKind::Press.
                // See: https://docs.rs/crossterm/0.26.1/crossterm/event/struct.KeyEvent.html#structfield.kind
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Down => self.arrow_down(),
                        KeyCode::Up => self.arrow_up(),
                        KeyCode::Enter => {
                            self.input_field.enter_message();

                            let input = self
                                .input_field
                                .last_message()
                                .expect("Expected input after pushing message to message buffer");

                            if let Ok(git_repo) = git2::Repository::open(&input) {
                                let mut url = String::new();

                                if let Ok(str_arr) = git_repo.remotes() {
                                    url = str_arr.get(0).unwrap_or("").to_owned();
                                }

                                let repo = Repository {
                                    path: input,
                                    name: git_repo.namespace().unwrap_or("UNAMED").to_owned(),
                                    repo_url: url,
                                };

                                self.repo_selected = Some(repo);
                            }
                        }

                        _ => {}
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
        let header_cells = ["Repo Name", "Path", "URL"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = self.repositories.table_items.iter().map(|item| {
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

        f.render_stateful_widget(table, rects[0], &mut self.repositories.table_state);

        let input_field_text = Paragraph::new(self.input_field.input.value())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Open folder")
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

    fn arrow_down(&mut self) {}

    fn arrow_up(&mut self) {}
}
