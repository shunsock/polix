use core::source_code::SourceCodeCharacter;
use core::token::Token;


pub struct Tokenizer {
    source: Vec<SourceCodeCharacter>,
    processed: Vec<Token>,
}