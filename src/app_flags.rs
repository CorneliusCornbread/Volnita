#[derive(Default)]
pub struct TerminateFlag {
    terminate_execution: bool,
}

impl TerminateFlag {
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
}
