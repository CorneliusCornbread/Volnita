use crate::commit_table::CommitTable;

use tui::{widgets::{TableState, Row, Table, Borders, Block, Cell}, style::{Style, Modifier, Color}, layout::{Constraint, Layout}};

use crate::{traits::display_view::{DisplayView}};

pub struct OpenedRepoView<'a> {
    pub repo_commits: CommitTable<'a>
}

impl<'a> OpenedRepoView<'a> {
    pub fn new() -> OpenedRepoView<'a> {
        let mut table = CommitTable {
            table_state: TableState::default(),
            table_items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row181", "Row182", "Row183"],
                vec!["Row191", "Row192", "Row193"],
            ],
        };

        table.table_state.select(Some(0));
        OpenedRepoView { repo_commits: table }
    }
}

impl DisplayView for OpenedRepoView<'_> {
    fn display_view<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) {
        let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["Header1", "Header2", "Header3"]
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
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);

        f.render_stateful_widget(t, rects[0], &mut self.repo_commits.table_state);
    }

    fn arrow_down(&mut self) {
        let i = match self.repo_commits.table_state.selected() {
            Some(i) => {
                let count = self.repo_commits.table_items.len();

                if count == 0 {
                    return
                }
                else if i >= count - 1 {
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
                    return
                }
                else if i == 0 {
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