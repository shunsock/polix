use std::process::exit;
use log::debug;
use scanner::Scanner;
use reporter::Reporter;
use core::polix_error_trait::PolixErrorTrait;

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
        let reporter = Reporter::new(self.src.clone());
        let scanner = Scanner::new(self.src.clone());
        let tokens= match scanner.scan() {
            Ok(tokens) => tokens,
            Err(e) => {
                reporter.report(
                    e.get_line(),
                    e.get_position(),
                    &format!("Scanner error: {}", e.get_message()),
                );
                exit(1);
            }
        };
        debug!("Source code scanner finished");
        for token in tokens.clone() {
            debug!("Token: {:?}", token);
        }
    }
}
