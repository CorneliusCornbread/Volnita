use tui::{widgets::{TableState}};

pub struct CommitTable<'a> {
    pub table_state: TableState,
    pub table_items: Vec<Vec<&'a str>>
}