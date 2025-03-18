use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;

use core::token::Literal::{
    Integer,
    Float,
};

pub fn recognize_literal_number(value: String, line: Line, position: Position) -> Option<Token> {
    if let Ok(int_value) = value.parse::<i64>() {
        Some(Token {
            kind: TokenKind::Literal(Integer(int_value)),
            line,
            position,
        })
    } else if let Ok(float_value) = value.parse::<f64>() {
        Some(Token {
            kind: TokenKind::Literal(Float(float_value)),
            line,
            position,
        })
    } else {
        None
    }
}
