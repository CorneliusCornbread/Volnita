use tui::widgets::TableState;

pub struct CommitTable {
    pub table_state: TableState,
    pub table_items: Vec<Vec<String>>,
}
