use buffer::Buffer;
use core::source_code::{Line, Position};
use core::token::raw_token::RawToken;

mod buffer;
mod buffer_text_updater;
mod line_updater;
mod token_generator;

#[derive(Clone, Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<RawToken>,
    pub buffer: Buffer,
    pub line: Line,
    pub position: Position,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            buffer: Buffer::new(),
            line: Line::new(1).unwrap(),
            position: Position::new(1).unwrap(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_empty()
    }
}

impl Scanner {
    pub fn scan(self) -> Scanner {
        // return if the all source has been consumed
        if self.clone().is_at_end() {
            return self;
        }

        // split characters into first character and rest of the characters
        // e.g. "abc" -> ('a', "bc")
        // calling unwrap() is safe here because we know that the string is not empty
        let first_char: char = self.source.chars().next().unwrap();
        let rest_characters: String = self.source.chars().skip(1).collect();

        // concat buffer and first character
        // e.g. buffer = "ab", first_char = "c" -> buffer_updated = "abc"
        let buffer_text_updated: Buffer = Buffer {
            text: buffer_text_updater::update(first_char, self.buffer.text.clone()),
            current_line: self.buffer.current_line,
            current_position: self.buffer.current_position,
            start_line: self.buffer.start_line,
            start_position: self.buffer.start_position,
        };

        // generate token
        let token: Option<RawToken> = token_generator::generate_token(
            first_char,
            rest_characters.clone(),
            self.buffer.clone(),
            self.line,
        );

        // update line and position
        // this update is for next iteration,
        // Because we initialized line and position with 1, 1
        let line_updated: Line = line_updater::update(first_char, self.line);
        let position_updated: Position = self.position.increment();
        let buffer_all_updated: Buffer = Buffer {
            text: buffer_text_updated.text,
            current_line: line_updated,
            current_position: position_updated,
            start_line: self.buffer.start_line,
            start_position: self.buffer.start_position,
        };

        let s_ = match token {
            Some(t) => {
                dbg!("[Scanner] Token generated: {:?}", t.clone());
                let tokens_updated: Vec<RawToken> =
                    self.tokens.into_iter().chain(std::iter::once(t)).collect();

                Scanner {
                    source: rest_characters,
                    tokens: tokens_updated,
                    buffer: Buffer::new(),
                    line: line_updated,
                    position: position_updated,
                }
            }
            None => Scanner {
                source: rest_characters,
                tokens: self.tokens.clone(),
                buffer: buffer_all_updated,
                line: line_updated,
                position: position_updated,
            },
        };
        s_.scan()
    }
}
