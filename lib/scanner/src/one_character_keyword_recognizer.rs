use core::source_code::SourceCodeCharacter;
use core::token::BinaryOperation;
use core::token::Colon;
use core::token::Keyword;
use core::token::Paren;
use core::token::Token;
use core::token::TokenKind;

/// Tokenizer is responsible for converting a list of SourceCodeCharacter into a list of Token.
///
/// example:
/// - before: "bind abc: int = 0;"
/// - after: unrecognized(bind) identifier(abc) colon(int) equal(=) integer(0) semicolon(;)
pub struct OneCharacterKeywordRecognizer {
    source: Vec<SourceCodeCharacter>,
    processed: Vec<Token>,
}

impl OneCharacterKeywordRecognizer {
    pub fn new(
        source: Vec<SourceCodeCharacter>,
        processed: Vec<Token>,
    ) -> OneCharacterKeywordRecognizer {
        OneCharacterKeywordRecognizer { source, processed }
    }

    pub fn recognize(&mut self) -> OneCharacterKeywordRecognizer {
        match self.source.len() {
            0 => OneCharacterKeywordRecognizer::new(vec![], self.processed.clone()),
            _ => {
                let mut source: Vec<SourceCodeCharacter> = self.source.clone();
                let mut processed: Vec<Token> = self.processed.clone();

                let first_char: SourceCodeCharacter = source.remove(0);
                match self.recognize_character(first_char.clone()) {
                    Some(token) => {
                        processed.push(token);
                    }
                    None => {
                        processed.push(Token::new(
                            TokenKind::Unrecognized(first_char.character.to_string()),
                            first_char.line,
                            first_char.position,
                        ));
                    }
                }

                OneCharacterKeywordRecognizer::new(source, processed).recognize()
            }
        }
    }

    fn recognize_character(&self, character: SourceCodeCharacter) -> Option<Token> {
        match character.character {
            '+' => Some(Token::new(
                TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Add)),
                character.line,
                character.position,
            )),
            '*' => Some(Token::new(
                TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Multiply)),
                character.line,
                character.position,
            )),
            '/' => Some(Token::new(
                TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Divide)),
                character.line,
                character.position,
            )),
            ':' => Some(Token::new(
                TokenKind::Keyword(Keyword::Colon(Colon::Colon)),
                character.line,
                character.position,
            )),
            ';' => Some(Token::new(
                TokenKind::Keyword(Keyword::Colon(Colon::SemiColon)),
                character.line,
                character.position,
            )),
            '[' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::SquareBracketLeft)),
                character.line,
                character.position,
            )),
            ']' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::SquareBracketRight)),
                character.line,
                character.position,
            )),
            '(' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::ParenLeft)),
                character.line,
                character.position,
            )),
            ')' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::ParenRight)),
                character.line,
                character.position,
            )),
            '{' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::BraceLeft)),
                character.line,
                character.position,
            )),
            '}' => Some(Token::new(
                TokenKind::Keyword(Keyword::Paren(Paren::BraceRight)),
                character.line,
                character.position,
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::source_code::Line;
    use core::source_code::Position;

    /// Creates a vector of SourceCodeCharacter from a string
    /// Each character will have a Line number of 1 and sequential Position numbers
    fn create_source_code_char_factory(text: String) -> Vec<SourceCodeCharacter> {
        let mut result = Vec::with_capacity(text.len());

        for (i, ch) in text.chars().enumerate() {
            let position = i + 1; // Position is 1-indexed
            result.push(SourceCodeCharacter::new(
                ch,
                Line::new(1).unwrap(),
                Position::new(position as u32).unwrap(),
            ));
        }

        result
    }

    #[test]
    /// # Test recognize pass with multiply
    ///
    /// ## Test Case:
    /// - input source code is: "*"
    /// - expected: multiply
    fn test_recognize_one_character_keyword() {
        // Arrange
        let source_text = String::from("*");
        let source_code: Vec<SourceCodeCharacter> =
            create_source_code_char_factory(source_text.clone());

        // Act
        let recognizer = OneCharacterKeywordRecognizer::new(source_code, vec![]).recognize();

        // Assert
        let expected: Vec<Token> = vec![Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Multiply)),
            Line::new(1).unwrap(),
            Position::new(1).unwrap(),
        )];
        assert_eq!(recognizer.processed, expected);
    }
}
