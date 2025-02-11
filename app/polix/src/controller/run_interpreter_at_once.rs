use log::debug;
use scanner::scanner::{scan, Scanner};

pub(crate) struct RunInterpreterAtOnce {
    src: String,
}

impl RunInterpreterAtOnce {
    pub(crate) fn new(src: String) -> Self {
        Self { src }
    }

    pub(crate) fn run(&self) {
        debug!("Running interpreter at once");
        debug!("Source code: {}", self.src.clone());
        let s = Scanner::new(self.src.clone());
        let s_scanned = scan(s).tokens;
        debug!("Scanned tokens: {:?}", s_scanned);
    }
}
