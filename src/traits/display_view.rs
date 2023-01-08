use tui::{backend::Backend, Frame};

pub trait DisplayView {
    fn display_view<B: Backend>(&mut self, f: &mut Frame<B>) -> bool;

    fn arrow_down(&mut self);

    fn arrow_up(&mut self);
}
