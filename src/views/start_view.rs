use std::{path::PathBuf, str::FromStr};

use crossterm::event::{KeyCode, KeyEventKind};

use tui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::{
    app_flags::AppLoopFlag,
    command::command_handler::CommandHandler,
    config::{
        repo::{SavedRepositories, SerializedRepository},
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
    pub repo_selected: Option<SerializedRepository>,
    arrow_used: bool,
}

impl StartView {
    pub fn load_table(&mut self) {
        self.repositories.table_items = Self::get_table_items();
    }

    fn get_table_items() -> Vec<Vec<String>> {
        let config = SavedRepositories::load_or_create_config();

        let mut table_items = Vec::new();
        for repo in config.recent_repositories {
            table_items.push(vec![
                repo.name,
                repo.path.to_string_lossy().to_string(),
                repo.repo_url,
            ])
        }

        table_items
    }
}

impl Default for StartView {
    fn default() -> Self {
        let mut view = Self {
            repositories: DataTable {
                table_state: TableState::default(),
                table_items: StartView::get_table_items(),
            },
            input_field: InputField::default(),
            force_draw: true,
            handler: CommandHandler::default(),
            repo_selected: None,
            arrow_used: true,
        };

        view.repositories.table_state.select(Some(0));
        view
    }
}

impl DisplayView for StartView {
    fn display_view<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) -> AppLoopFlag {
        if !self.force_draw {
            if let Some(key_event) = self.input_field.input_wait() {
                if input_field::is_quit_event(&key_event) {
                    return AppLoopFlag::terminate();
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
                            if self.arrow_used {
                                let selected_ind =
                                    self.repositories.table_state.selected().unwrap_or_default();
                                if let Some(selected_repo) =
                                    self.repositories.table_items.get(selected_ind)
                                {
                                    let repo = SerializedRepository {
                                        path: PathBuf::from_str(
                                            selected_repo.get(1).unwrap_or(&String::new()),
                                        )
                                        .unwrap_or_default(),
                                        name: selected_repo
                                            .get(0)
                                            .unwrap_or(&String::new())
                                            .to_string(),
                                        repo_url: selected_repo
                                            .get(2)
                                            .unwrap_or(&String::new())
                                            .to_string(),
                                    };

                                    self.repo_selected = Some(repo);
                                    return AppLoopFlag::terminate();
                                }
                            } else {
                                self.input_field.enter_message();

                                let input = self.input_field.last_message().expect(
                                    "Expected input after pushing message to message buffer",
                                );

                                if let Ok(git_repo) = git2::Repository::open(&input) {
                                    let mut url = String::new();

                                    if let Ok(str_arr) = git_repo.remotes() {
                                        url = str_arr.get(0).unwrap_or("").to_owned();
                                    }
                                    let folders: Vec<&str> = input.split('/').collect();

                                    let recent_repo = crate::config::repo::SerializedRepository {
                                        path: PathBuf::from_str(&input).unwrap_or_default(),
                                        name: folders
                                            .get(folders.len() - 2)
                                            .unwrap_or(&"UNNAMED")
                                            .to_string(),
                                        repo_url: url,
                                    };

                                    self.repo_selected = Some(recent_repo);

                                    return AppLoopFlag::terminate();
                                }
                            }
                        }

                        _ => {
                            self.arrow_used = false;
                        }
                    }
                }
            }
        } else {
            self.force_draw = false;
        }

        let rects = Layout::default()
            .constraints([Constraint::Max(80), Constraint::Min(4)].as_ref())
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

        AppLoopFlag::continue_()
    }

    fn arrow_down(&mut self) {
        let i = match self.repositories.table_state.selected() {
            Some(i) => {
                let count = self.repositories.table_items.len();

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
        self.repositories.table_state.select(Some(i));
        self.arrow_used = true;
    }

    fn arrow_up(&mut self) {
        let i = match self.repositories.table_state.selected() {
            Some(i) => {
                let count = self.repositories.table_items.len();

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
        self.repositories.table_state.select(Some(i));
        self.arrow_used = true;
    }
}
