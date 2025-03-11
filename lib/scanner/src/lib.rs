mod comment_remover;
mod source_stream_generator;
mod tokenizer;

use comment_remover::CommentRemover;
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
        let comment_remover =
            CommentRemover::new(stream_generator.get_processed(), vec![]).remove(false);
        debug!("{:?}", comment_remover.get_processed());
    }
}
