use core::token::token_with_parsed_identifier::TokenWithParsedIdentifierType;
use core::source_code::{Line, Position};
use crate::searcher::{search_keyword, search_number};
use crate::transformer_error::TransformerError;
use crate::parser::{parse_float, parse_integer, parse_string};

pub fn route(s: String, l: Line, p: Position) -> Result<TokenWithParsedIdentifierType, TransformerError> {
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
        },
        false => (),
    }

    parse_string(s, l, p)
}
