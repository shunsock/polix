mod source_stream_generator;
mod comment_remover;

use log::debug;
use source_stream_generator::SourceStreamGenerator;

pub struct Scanner {
    source_code: Vec<char>,
}

impl Scanner {
    pub fn new(source_code: String) -> Self {
        Scanner {
            source_code: source_code.chars().collect(),
        }
    }

    pub fn scan(&self) {
        let stream_generator =
            SourceStreamGenerator::new(self.source_code.clone(), vec![], None, None).generate();
        debug!("{:?}", stream_generator.get_processed());
    }
}
