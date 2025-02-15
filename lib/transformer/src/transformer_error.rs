use core::source_code::Line;
use core::source_code::Position;
use core::polix_error_trait::PolixErrorTrait;

pub struct TransformerError {
    pub error_kind: TransformerErrorKind,
    pub line: Line,
    pub position: Position,
}

pub enum TransformerErrorKind {
    InvalidFloatLiteralFound,
    InvalidIntegerLiteralFound,
    InvalidStringLiteralFound,
}

impl TransformerError {
    pub fn new(error_kind: TransformerErrorKind, line: Line, position: Position) -> TransformerError {
        TransformerError {
            error_kind,
            line,
            position,
        }
    }
}

impl PolixErrorTrait for TransformerError {
    fn get_message(&self) -> String {
        match self.error_kind {
            TransformerErrorKind::InvalidFloatLiteralFound => "Invalid float literal found".to_string(),
            TransformerErrorKind::InvalidIntegerLiteralFound => "Invalid integer literal found".to_string(),
            TransformerErrorKind::InvalidStringLiteralFound => "Invalid string literal found".to_string(),
        }
    }

    fn get_line(&self) -> Line {
        self.line.clone()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}