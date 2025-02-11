use crate::source_code::Line;
use crate::source_code::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct RawToken {
    pub token_type: RawTokenType,
    pub line: Line,
    pub position: Position,
}

impl RawToken {
    pub fn new(token_type: RawTokenType, line: Line, position: Position) -> RawToken {
        RawToken {
            token_type,
            line,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RawTokenType {
    AngleLeft,
    AngleRight,
    Asterisk,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    Colon,
    Comma,
    Dot,
    Eof,
    Identifier(String),
    ParenthesisLeft,
    ParenthesisRight,
    Percent,
    Plus,
    Semicolon,
}
