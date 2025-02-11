pub mod token_type;

use token_type::TokenType;
use crate::source_code::line::Line;
use crate::source_code::position::Position;

pub(super) struct Token {
    pub token_type: TokenType,
    pub line: Line,
    pub position: Position,
}
