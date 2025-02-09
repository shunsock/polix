use log::debug;

pub(crate) struct RunInterpreterInteractively {}

impl RunInterpreterInteractively {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn run(&self) {
        debug!("Running interpreter interactively");
    }
}
