mod identifier;

use core::token::Token;
use core::source_code::{ Line, Position };


/// generate token from the given character, rest of the source code and buffer
///
/// - return None when c is whitespace
/// - return Identifier token with buffer + c when c is alphabetic and rest starts with whitespace
pub(crate) fn generate_token(
    c: char,
    rest: String,
    buffer: String,
    line: Line,
    position: Position
) -> Option<Token> {
    match c {
        '\0' | '\t' | '\r' | ' ' => {
            None
        }
        '\n' => {
            line.increment();
            None
        }
        'a'..='z' | 'A'..='Z' | '_' => {
            identifier::generate(rest, buffer, line, position)
        }
        _ => None,
    }
}
