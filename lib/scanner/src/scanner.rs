use buffer::Buffer;
use core::source_code::{Line, Position};
use core::token::raw_token::RawToken;

mod buffer;
mod buffer_text_updater;
mod line_updater;
mod position_updater;
mod token_generator;

#[derive(Clone, Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<RawToken>,
    pub buffer: Buffer,
    pub line: Line,
    pub position: Position,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            buffer: Buffer::new(),
            line: Line::new(1).unwrap(),
            position: Position::new(1).unwrap(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_empty()
    }
}

impl Scanner {
    pub fn scan(self) -> Scanner {
        // return if the all source has been consumed
        if self.clone().is_at_end() {
            return self;
        }

        // split characters into first character and rest of the characters
        // e.g. "abc" -> ('a', "bc")
        // calling unwrap() is safe here because we know that the string is not empty
        let first_char: char = self.source.chars().next().unwrap();
        let rest_characters: String = self.source.chars().skip(1).collect();

        // concat buffer and first character
        // e.g. buffer = "ab", first_char = "c" -> buffer_updated = "abc"
        let buffer_text_updated: Buffer = Buffer {
            text: buffer_text_updater::update(first_char, self.buffer.text.clone()),
            current_line: self.buffer.current_line,
            current_position: self.buffer.current_position,
            start_line: self.buffer.start_line,
            start_position: self.buffer.start_position,
        };

        // generate token
        let token: Option<RawToken> = token_generator::generate_token(
            first_char,
            rest_characters.clone(),
            buffer_text_updated.clone(),
            self.line,
        );

        // update line and position
        // this update is for next iteration,
        // Because we initialized line and position with 1, 1
        let line_updated: Line = line_updater::update(first_char, self.line);
        let position_updated: Position = position_updater::update(first_char, self.position);
        let buffer_all_updated: Buffer = Buffer {
            text: buffer_text_updated.text.clone(),
            current_line: line_updated,
            current_position: position_updated,
            start_line: self.buffer.start_line,
            start_position: self.buffer.start_position,
        };

        let s_ = match token {
            Some(t) => {
                dbg!("[Scanner] Token generated: {:?}", t.clone());
                let tokens_updated: Vec<RawToken> =
                    self.tokens.into_iter().chain(std::iter::once(t)).collect();

                Scanner {
                    source: rest_characters,
                    tokens: tokens_updated,
                    buffer: Buffer {
                        text: String::new(),
                        current_line: line_updated,
                        current_position: position_updated,
                        start_line: line_updated,
                        start_position: position_updated,
                    },
                    line: line_updated,
                    position: position_updated,
                }
            }
            None => Scanner {
                source: rest_characters,
                tokens: self.tokens.clone(),
                buffer: buffer_all_updated,
                line: line_updated,
                position: position_updated,
            },
        };
        s_.scan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::source_code::Line;
    use core::token::raw_token::RawTokenType;

    /// Test: When the source string is empty,
    /// the scan() method should return a Scanner with an empty source and no tokens.
    #[test]
    fn test_scan_empty_source() {
        // Given: an empty source string.
        let scanner = Scanner::new(String::new());

        // When: we call scan() on the scanner.
        let result = scanner.scan();

        // Then: the returned Scanner should have an empty source and no tokens.
        assert!(
            result.source.is_empty(),
            "Source should be empty after scanning."
        );
        assert!(
            result.tokens.is_empty(),
            "No tokens should be produced for an empty source."
        );
    }

    /// Test: When the source contains a single token ending with a whitespace,
    /// scan() should produce one token with the accumulated text.
    #[test]
    fn test_scan_single_token() {
        // Given: a source string that ends with a whitespace to trigger token generation.
        let input = "abc ";
        let scanner = Scanner::new(input.to_string());

        // When: we call scan().
        let result = scanner.scan();

        // Then: one token with the value "abc" should be produced.
        assert!(
            result.source.is_empty(),
            "Source should be empty after scanning."
        );
        assert_eq!(
            result.tokens.len(),
            1,
            "Exactly one token should be generated."
        );

        // 直接 tokens[0] は RawToken であるため Option として扱わず、token.token_type を確認する。
        let token = &result.tokens[0];
        match &token.token_type {
            RawTokenType::Identifier(s) => {
                assert_eq!(s, "abc", "The token value should be 'abc'.");
            }
            other => {
                panic!("Unexpected token type: {:?}", other);
            }
        }

        // Additionally, check that the line number remains 1.
        assert_eq!(
            result.line,
            Line::new(1).unwrap(),
            "The final line number should be 1."
        );
    }

    /// Test: When the source contains multiple tokens separated by whitespace,
    /// scan() should produce tokens accordingly.
    #[test]
    fn test_scan_multiple_tokens() {
        // Given: a source string with two tokens separated by a space.
        let input = "hello world ";
        let scanner = Scanner::new(input.to_string());

        // When: we call scan().
        let result = scanner.scan();

        // Then: two tokens should be produced: "hello" and "world".
        assert!(
            result.source.is_empty(),
            "Source should be empty after scanning."
        );
        assert_eq!(result.tokens.len(), 2, "Two tokens should be generated.");

        // Check first token:
        let token0 = &result.tokens[0];
        match &token0.token_type {
            RawTokenType::Identifier(s) => {
                assert_eq!(s, "hello", "The first token should be 'hello'.");
            }
            other => {
                panic!("Unexpected token type for first token: {:?}", other);
            }
        }

        // Check second token:
        let token1 = &result.tokens[1];
        match &token1.token_type {
            RawTokenType::Identifier(s) => {
                assert_eq!(s, "world", "The second token should be 'world'.");
            }
            other => {
                panic!("Unexpected token type for second token: {:?}", other);
            }
        }
    }

    /// Test: When the source contains newline characters,
    /// scan() should update the line count and produce tokens per line.
    #[test]
    fn test_scan_with_newline() {
        // Given: a source string with a newline character separating two tokens.
        let input = "line1\nline2 ";
        let scanner = Scanner::new(input.to_string());

        // When: we call scan().
        let result = scanner.scan();

        // Then: two tokens should be produced: "line1" and "line2".
        assert!(
            result.source.is_empty(),
            "Source should be empty after scanning."
        );
        assert_eq!(
            result.tokens.len(),
            2,
            "Two tokens should be generated when newline is present."
        );

        // Check first token:
        let token0 = &result.tokens[0];
        match &token0.token_type {
            RawTokenType::Identifier(s) => {
                assert_eq!(s, "line1", "The first token should be 'line1'.");
            }
            other => {
                panic!(
                    "Unexpected token type for first token with newline: {:?}",
                    other
                );
            }
        }

        // Check second token:
        let token1 = &result.tokens[1];
        match &token1.token_type {
            RawTokenType::Identifier(s) => {
                assert_eq!(s, "line2", "The second token should be 'line2'.");
            }
            other => {
                panic!(
                    "Unexpected token type for second token with newline: {:?}",
                    other
                );
            }
        }

        // And: the line number should be updated (assuming the newline increments the line count).
        assert_eq!(
            result.line,
            Line::new(2).unwrap(),
            "The final line number should be 2 after scanning a newline."
        );
    }
}
