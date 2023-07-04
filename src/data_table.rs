use tui::widgets::TableState;

#[derive(Default)]
pub struct DataTable {
    pub table_state: TableState,
    pub table_items: Vec<Vec<String>>,
}
