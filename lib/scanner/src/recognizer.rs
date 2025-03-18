mod keyword;

use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;

use crate::scanner_error::{ScannerError, ScannerErrorKind};
use keyword::recognize_keyword;

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

    pub fn recognize(self) -> Self {
        let mut source = self.source.clone();
        let mut processed = self.processed.clone();

        while !source.is_empty() {
            let t = source.remove(0);
            match t.kind {
                TokenKind::Unrecognized(value) => {
                    let token = Token::new(TokenKind::Unrecognized(value), t.line, t.position);
                    processed.push(token);
                }
                _ => {
                    processed.push(t);
                }
            }
        }

        Recognizer { source, processed }
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

        Err(ScannerError::new(
            ScannerErrorKind::UnrecognizedToken,
            start_line,
            start_position,
        ))
    }
}
