mod comment_remover;
mod multiple_space_remover;
mod one_character_keyword_recognizer;
mod source_stream_generator;
mod tokenizer;

use comment_remover::CommentRemover;
use log::debug;
use multiple_space_remover::MultipleSpaceRemover;
use one_character_keyword_recognizer::OneCharacterKeywordRecognizer;
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
        debug!("Scanning source code: Generating source stream...");
        let stream_generator =
            SourceStreamGenerator::new(self.source_code.clone(), vec![], None, None).generate();
        for s in stream_generator.get_processed() {
            debug!("{:?}", s);
        }

        debug!("Scanning source code: Removing multiple spaces...");
        let multiple_space_remover =
            MultipleSpaceRemover::new(stream_generator.get_processed(), vec![]).remove(false);
        for s in multiple_space_remover.get_processed() {
            debug!("{:?}", s);
        }

        debug!("Scanning source code: Removing comments...");
        let comment_remover =
            CommentRemover::new(multiple_space_remover.get_processed(), vec![]).remove(false);
        for s in comment_remover.get_processed() {
            debug!("{:?}", s);
        }

        debug!("Scanning source code: Recognizing one-character keywords...");
        let one_character_keyword_recognizer =
            OneCharacterKeywordRecognizer::new(comment_remover.get_processed(), vec![]).recognize();
        for s in one_character_keyword_recognizer.get_processed() {
            debug!("{:?}", s);
        }

        debug!("Scanning source code: Done!");
    }
}
