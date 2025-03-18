mod identifier;
mod keyword;
mod literal_number;
mod literal_string;

use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;

use crate::recognizer::identifier::recognize_identifier;
use crate::recognizer::literal_number::recognize_literal_number;
use crate::scanner_error::{ScannerError, ScannerErrorKind};
use keyword::recognize_keyword;
use literal_string::recognize_literal_string;

pub struct Recognizer {
    source: Vec<Token>,
    processed: Vec<Token>,
}

impl Recognizer {
    pub fn new(source: Vec<Token>) -> Self {
        Recognizer {
            source,
            processed: Vec::new(),
        }
    }

    pub fn get_processed(&self) -> Vec<Token> {
        self.processed.clone()
    }

    pub fn recognize(self) -> Result<Self, ScannerError> {
        let mut source = self.source.clone();
        let mut processed = self.processed.clone();

        while !source.is_empty() {
            let t = source.remove(0);
            match t.kind {
                TokenKind::Unrecognized(value) => {
                    let token = Self::recognize_token(value.clone(), t.line, t.position)?;
                    processed.push(token);
                }
                _ => {
                    processed.push(t);
                }
            }
        }

        Ok(Recognizer { source, processed })
    }

    fn recognize_token(
        value: String,
        start_line: Line,
        start_position: Position,
    ) -> Result<Token, ScannerError> {
        match recognize_keyword(value.clone(), start_line, start_position) {
            Some(token) => return Ok(token),
            _ => {}
        }

        match recognize_literal_number(value.clone(), start_line, start_position) {
            Some(token) => return Ok(token),
            _ => {}
        }

        match recognize_literal_string(value.clone(), start_line, start_position) {
            Some(token) => return Ok(token),
            _ => {}
        }

        match recognize_identifier(value.clone(), start_line, start_position) {
            Some(token) => return Ok(token),
            _ => {}
        }

        Err(ScannerError::new(
            ScannerErrorKind::UnrecognizedToken,
            start_line,
            start_position,
        ))
    }
}
