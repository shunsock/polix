mod identifier;

use crate::buffer::Buffer;
use core::source_code::Line;
use core::token::token::Token;

/// generate token from the given character, rest of the source code and buffer
///
/// - return None when c is whitespace
/// - return Identifier token with buffer + c when c is alphabetic and rest starts with whitespace
pub(crate) fn generate_token(c: char, rest: String, buffer: Buffer, line: Line) -> Option<Token> {
    match c {
        '\0' | '\t' | '\r' | ' ' => None,
        '\n' => {
            line.increment();
            None
        }
        'a'..='z' | 'A'..='Z' | '_' => identifier::generate(rest, buffer),
        _ => None,
    }
}
