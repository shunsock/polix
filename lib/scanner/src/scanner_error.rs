use core::polix_error_trait::PolixErrorTrait;
use core::source_code::{Line, Position};

pub struct ScannerError {
    message: String,
    line: Line,
    position: Position,
}

pub enum ScannerErrorKind {
    UnrecognizedToken,
    InvalidCharacter,
    UnexpectedEndOfInput,
}

impl ScannerError {
    pub fn new(kind: ScannerErrorKind, line: Line, position: Position) -> Self {
        ScannerError {
            message: Self::kind_to_message(kind),
            line,
            position,
        }
    }

    fn kind_to_message(kind: ScannerErrorKind) -> String {
        match kind {
            ScannerErrorKind::UnrecognizedToken => "Unrecognized token".to_string(),
            ScannerErrorKind::InvalidCharacter => "Invalid character".to_string(),
            ScannerErrorKind::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
        }
    }
}

impl PolixErrorTrait for ScannerError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_line(&self) -> Line {
        self.line
    }

    fn get_position(&self) -> Position {
        self.position
    }
}
