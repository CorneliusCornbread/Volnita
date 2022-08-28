use tui::{Frame, backend::Backend};

pub trait DisplayView {
    fn display_view<B: Backend>(&mut self, f: &mut Frame<B>);
}