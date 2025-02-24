use core::polix_error_trait::PolixErrorTrait;
use core::source_code::Line;
use core::source_code::Position;

pub struct IdentifierExtractorError {
    pub error_kind: IdentifierExtractorErrorKind,
    pub line: Line,
    pub position: Position,
}

pub enum IdentifierExtractorErrorKind {
    InvalidFloatLiteralFound,
    InvalidIntegerLiteralFound,
    InvalidStringLiteralFound,
}

impl IdentifierExtractorError {
    pub fn new(
        error_kind: IdentifierExtractorErrorKind,
        line: Line,
        position: Position,
    ) -> IdentifierExtractorError {
        IdentifierExtractorError {
            error_kind,
            line,
            position,
        }
    }
}

impl PolixErrorTrait for IdentifierExtractorError {
    fn get_message(&self) -> String {
        match self.error_kind {
            IdentifierExtractorErrorKind::InvalidFloatLiteralFound => {
                "Invalid float literal found".to_string()
            }
            IdentifierExtractorErrorKind::InvalidIntegerLiteralFound => {
                "Invalid integer literal found".to_string()
            }
            IdentifierExtractorErrorKind::InvalidStringLiteralFound => {
                "Invalid string literal found".to_string()
            }
        }
    }

    fn get_line(&self) -> Line {
        self.line
    }

    fn get_position(&self) -> Position {
        self.position
    }
}
