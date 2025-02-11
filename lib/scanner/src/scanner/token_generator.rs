mod identifier;

use crate::scanner::buffer::Buffer;
use core::source_code::Line;
use core::token::raw_token::{RawToken, RawTokenType};

/// generate token from the given character, rest of the source code and buffer
///
/// - return None when c is whitespace
/// - return Identifier token with buffer + c when c is alphabetic and rest starts with whitespace
pub(crate) fn generate_token(
    c: char,
    rest: String,
    buffer: Buffer,
    line: Line,
) -> Option<RawToken> {
    match c {
        '\0' | '\t' | '\r' | ' ' => None,
        '\n' => {
            line.increment();
            None
        },
        '<' => Some(RawToken::new(
            RawTokenType::AngleLeft,
            line,
            buffer.current_position,
        )),
        '>' => Some(RawToken::new(
            RawTokenType::AngleRight,
            line,
            buffer.current_position,
        )),
        '*' => Some(RawToken::new(
            RawTokenType::Asterisk,
            line,
            buffer.current_position,
        )),
        '{' => Some(RawToken::new(
            RawTokenType::BraceLeft,
            line,
            buffer.current_position,
        )),
        '}' => Some(RawToken::new(
            RawTokenType::BraceRight,
            line,
            buffer.current_position,
        )),
        '[' => Some(RawToken::new(
            RawTokenType::BracketLeft,
            line,
            buffer.current_position,
        )),
        ']' => Some(RawToken::new(
            RawTokenType::BracketRight,
            line,
            buffer.current_position,
        )),
        ':' => Some(RawToken::new(
            RawTokenType::Colon,
            line,
            buffer.current_position,
        )),
        ',' => Some(RawToken::new(
            RawTokenType::Comma,
            line,
            buffer.current_position,
        )),
        '.' => Some(RawToken::new(
            RawTokenType::Dot,
            line,
            buffer.current_position,
        )),
        '(' => Some(RawToken::new(
            RawTokenType::ParenthesisLeft,
            line,
            buffer.current_position,
        )),
        ')' => Some(RawToken::new(
            RawTokenType::ParenthesisRight,
            line,
            buffer.current_position,
        )),
        '%' => Some(RawToken::new(
            RawTokenType::Percent,
            line,
            buffer.current_position,
        )),
        '+' => Some(RawToken::new(
            RawTokenType::Plus,
            line,
            buffer.current_position,
        )),
        ';' => Some(RawToken::new(
            RawTokenType::Semicolon,
            line,
            buffer.current_position,
        )),
        _ => identifier::generate(rest, buffer),
    }
}
