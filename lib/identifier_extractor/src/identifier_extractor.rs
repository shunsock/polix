use crate::identifier_extractor_error::IdentifierExtractorError;
use crate::router::route;
use core::token::raw_token::RawToken;
use core::token::token_with_parsed_identifier::{
    TokenWithParsedIdentifier, TokenWithParsedIdentifierType,
};

pub struct ExtractorDto {
    pub tokens_before_processed: Vec<RawToken>,
    pub tokens_processed: Vec<TokenWithParsedIdentifier>,
}

impl ExtractorDto {
    pub fn new(tokens: Vec<RawToken>) -> ExtractorDto {
        ExtractorDto {
            tokens_before_processed: tokens,
            tokens_processed: vec![],
        }
    }
}

fn extract(dto: ExtractorDto) -> Result<ExtractorDto, IdentifierExtractorError> {
    match dto.tokens_before_processed.split_first() {
        Some((first_token, rest_tokens)) => {
            let first_token_type: TokenWithParsedIdentifierType = route(first_token.clone())?;
            let mut v: Vec<TokenWithParsedIdentifier> = dto.tokens_processed.clone();
            v.push(TokenWithParsedIdentifier {
                token_type: first_token_type,
                line: first_token.line,
                position: first_token.position,
            });

            extract(ExtractorDto {
                tokens_before_processed: rest_tokens.to_vec(),
                tokens_processed: v,
            })
        }
        None => Ok(dto),
    }
}
