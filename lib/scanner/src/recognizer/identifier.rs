use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;
use regex::Regex;
use core::token::Identifier::{Struct, Function, Variable, Module};


pub fn recognize_identifier(value: String, line: Line, position: Position) -> Option<Token> {
    let struct_regex = Regex::new(r"^[A-Z][a-zA-Z]+$").unwrap();
    let function_regex = Regex::new(r"^[a-z][a-zA-Z0-9_]+$").unwrap();
    let variable_regex = Regex::new(r"^[a-z][a-z0-9_]+$").unwrap();
    let module_regex = Regex::new(r"^[a-z]+$").unwrap();

    if struct_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(Struct(value)),
            line,
            position,
        })
    } else if function_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(Function(value)),
            line,
            position,
        })
    } else if variable_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(Variable(value)),
            line,
            position,
        })
    } else if module_regex.is_match(&value) {
        Some(Token {
            kind: TokenKind::Identifier(Module(value)),
            line,
            position,
        })
    } else {
        None
    }
}