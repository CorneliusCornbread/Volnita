use tui::{backend::Backend, Frame};

use crate::app_flags::AppLoopFlag;

pub trait DisplayView {
    fn display_view<B: Backend>(&mut self, f: &mut Frame<B>) -> AppLoopFlag;

    fn arrow_down(&mut self);

    fn arrow_up(&mut self);
}
