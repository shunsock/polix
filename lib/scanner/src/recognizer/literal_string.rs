use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;
use core::token::Literal::Text;

pub fn recognize_literal_string(value: String, line: Line, position: Position) -> Option<Token> {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() >= 2 && chars[0] == '"' && chars[chars.len() - 1] == '"' {
        let inner_content: String = chars[1..chars.len() - 1].iter().collect();

        Some(Token {
            kind: TokenKind::Literal(Text(inner_content)),
            line,
            position,
        })
    } else {
        None
    }
}
