#[derive(Default)]
pub struct AppLoopFlag {
    terminate_execution: bool,
}

impl AppLoopFlag {
    pub fn should_continue(&self) -> bool {
        !self.terminate_execution
    }

    pub fn should_terminate(&self) -> bool {
        self.terminate_execution
    }

    pub fn new(terminate: bool) -> Self {
        Self {
            terminate_execution: terminate,
        }
    }

    pub fn terminate() -> Self {
        Self {
            terminate_execution: true,
        }
    }

    pub fn continue_() -> Self {
        Self {
            terminate_execution: false,
        }
    }
}
