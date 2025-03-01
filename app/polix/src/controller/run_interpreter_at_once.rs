use core::polix_error_trait::PolixErrorTrait;
use core::token::raw_token::RawToken;
use identifier_extractor::identifier_extractor::{extract, ExtractorDto};
use log::debug;
use reporter::Reporter;
use scanner::scanner::Scanner;
use std::process::exit;

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
        let raw_tokens: Vec<RawToken> = Scanner::new(self.src.clone()).scan().tokens;
        debug!("Raw tokens: {:?}", raw_tokens);
        let token_preprocessed = match extract(ExtractorDto::new(raw_tokens)) {
            Ok(dto) => dto.tokens_processed,
            Err(e) => {
                let r = Reporter::new(self.src.clone());
                r.report(e.get_line(), e.get_position(), e.get_message().as_str());
                exit(1)
            }
        };
        debug!("Token preprocessed: {:?}", token_preprocessed);
    }
}
