use tui::{Frame, backend::Backend};

pub trait DisplayView {
    fn display_view<B: Backend>(&mut self, f: &mut Frame<B>);

    fn arrow_down(&mut self);

    fn arrow_up(&mut self);
}