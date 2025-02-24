use crate::identifier_extractor_error::IdentifierExtractorError;
use crate::parser::{parse_float, parse_integer, parse_string};
use crate::searcher::{search_keyword, search_number};
use core::source_code::{Line, Position};
use core::token::raw_token::{RawToken, RawTokenType};
use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;

pub fn route(token: RawToken) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match token.token_type {
        RawTokenType::Identifier(string) => route_identifier(string, token.line, token.position),
        _ => Ok(route_others(token.token_type)),
    }
}

pub fn route_identifier(
    s: String,
    l: Line,
    p: Position,
) -> Result<TokenWithParsedIdentifierType, IdentifierExtractorError> {
    match search_keyword(s.clone()) {
        Some(t) => return Ok(t),
        None => (),
    }

    match search_number(s.clone()) {
        true => {
            return if s.contains('.') {
                parse_float(s, l, p)
            } else {
                parse_integer(s, l, p)
            }
        }
        false => (),
    }

    parse_string(s, l, p)
}

fn route_others(token_type: RawTokenType) -> TokenWithParsedIdentifierType {
    match token_type {
        RawTokenType::AngleLeft => TokenWithParsedIdentifierType::DelimiterAngleLeft,
        RawTokenType::AngleRight => TokenWithParsedIdentifierType::DelimiterAngleRight,
        RawTokenType::BraceLeft => TokenWithParsedIdentifierType::DelimiterBraceLeft,
        RawTokenType::BraceRight => TokenWithParsedIdentifierType::DelimiterBraceRight,
        RawTokenType::BracketLeft => TokenWithParsedIdentifierType::DelimiterBracketLeft,
        RawTokenType::BracketRight => TokenWithParsedIdentifierType::DelimiterBracketRight,
        RawTokenType::ParenthesisLeft => TokenWithParsedIdentifierType::DelimiterParenthesisLeft,
        RawTokenType::ParenthesisRight => TokenWithParsedIdentifierType::DelimiterParenthesisRight,
        RawTokenType::Colon => TokenWithParsedIdentifierType::SeparatorColon,
        RawTokenType::Comma => TokenWithParsedIdentifierType::SeparatorComma,
        RawTokenType::Dot => TokenWithParsedIdentifierType::SeparatorDot,
        RawTokenType::Percent => TokenWithParsedIdentifierType::OperatorPercent,
        RawTokenType::Plus => TokenWithParsedIdentifierType::OperatorPlus,
        RawTokenType::Semicolon => TokenWithParsedIdentifierType::SeparatorSemicolon,
        _ => panic!("Invalid token type"),
    }
}
