//! Source stream generator module.
//!
//! This module provides functionality to convert raw source code characters into a stream of
//! `SourceCodeCharacter` instances with line and position tracking.

use core::source_code::Line;
use core::source_code::Position;
use core::source_code::SourceCodeCharacter;

/// A generator that transforms raw source code into a stream of characters with position information.
///
/// The `SourceStreamGenerator` processes source code character by character, tracking line numbers
/// and positions. It handles line breaks by incrementing the line counter and resetting the position.
#[derive(Debug, PartialEq, Clone)]
pub struct SourceStreamGenerator {
    rest_source_code: Vec<char>,
    processed: Vec<SourceCodeCharacter>,
    line: u32,
    position: u32,
}

impl SourceStreamGenerator {
    /// Creates a new `SourceStreamGenerator` instance.
    ///
    /// # Parameters
    ///
    /// * `rest_source_code` - The raw source code characters to be processed
    /// * `processed` - Any previously processed source code characters
    /// * `index` - The current line number (defaults to 1 if None)
    /// * `position` - The current character position within the line (defaults to 1 if None)
    ///
    /// # Returns
    ///
    /// A new `SourceStreamGenerator` configured with the provided parameters
    pub fn new(
        rest_source_code: Vec<char>,
        processed: Vec<SourceCodeCharacter>,
        index: Option<u32>,
        position: Option<u32>,
    ) -> SourceStreamGenerator {
        match (index, position) {
            (Some(l), Some(p)) => SourceStreamGenerator {
                rest_source_code,
                processed,
                line: l,
                position: p,
            },
            _ => SourceStreamGenerator {
                rest_source_code,
                processed,
                line: 1,
                position: 1,
            },
        }
    }

    /// Processes the source code and generates a stream of characters with position information.
    ///
    /// This method recursively processes each character in the source code, creating
    /// `SourceCodeCharacter` objects that include line and position information. It handles
    /// newlines by incrementing the line counter and resetting the position counter.
    ///
    /// # Returns
    ///
    /// A new `SourceStreamGenerator` with all characters processed
    pub fn generate(&self) -> Self {
        match self.rest_source_code.len() {
            0 => self.clone(),
            _ => {
                // Get the first character from the source code
                // This unwrap is ok, because we are sure that the vec is not empty
                let mut rest_chars: Vec<char> = self.rest_source_code.clone();
                let first_char: char = rest_chars.remove(0);

                // create a new SourceCodeCharacter
                // These unwraps are ok, because we are sure that the line and position are greater than 0
                let source_code_char = SourceCodeCharacter::new(
                    first_char,
                    Line::new(self.line).unwrap(),
                    Position::new(self.position).unwrap(),
                );

                // push the new SourceCodeCharacter to the stream
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                processed.push(source_code_char);

                // if the first character is a newline character, increment the line and reset the position
                if first_char == '\n' {
                    return SourceStreamGenerator::new(
                        rest_chars,
                        processed,
                        Some(self.line + 1),
                        Some(1),
                    )
                    .generate();
                }

                // create a new StreamCreator with the rest of the source code
                SourceStreamGenerator::new(
                    rest_chars,
                    processed,
                    Some(self.line),
                    Some(self.position + 1),
                )
                .generate()
            }
        }
    }

    /// Returns the processed stream of source code characters.
    ///
    /// # Returns
    ///
    /// A vector of `SourceCodeCharacter` objects representing the processed source code
    pub fn get_processed(&self) -> Vec<SourceCodeCharacter> {
        self.processed.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// # Test StreamCreator parse a source code without \n
    /// The source code is `['a', 'b', 'c']`
    /// The expected result is a stream with 3 SourceCodeCharacter
    /// 1. SourceCodeCharacter { character: 'a', line: 1, position: 1 }
    /// 2. SourceCodeCharacter { character: 'b', line: 1, position: 2 }
    /// 3. SourceCodeCharacter { character: 'c', line: 1, position: 3 }
    fn test_parse_source_code_without_newline() {
        // Arrange
        let source_code: Vec<char> = vec!['a', 'b', 'c'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('b', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('c', Line::new(1).unwrap(), Position::new(3).unwrap()),
        ];

        assert_eq!(stream_creator.processed, expected);
    }

    #[test]
    /// # Test StreamCreator can increment line
    /// The source code is `['a', '\n', 'b', 'c']`
    /// The expected result is a stream with 3 SourceCodeCharacter
    /// 1. SourceCodeCharacter { character: 'a', line: 1, position: 1 }
    /// 2. SourceCodeCharacter { character: '\n', line: 1, position: 2 }
    /// 3. SourceCodeCharacter { character: 'b', line: 2, position: 1 }
    /// 4. SourceCodeCharacter { character: 'c', line: 2, position: 2 }
    fn test_parse_source_code_with_newline() {
        // Arrange
        let source_code: Vec<char> = vec!['a', '\n', 'b', 'c'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('\n', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('b', Line::new(2).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('c', Line::new(2).unwrap(), Position::new(2).unwrap()),
        ];

        assert_eq!(stream_creator.processed, expected);
    }

    #[test]
    /// # Test StreamCreator with empty source code
    /// The source code is empty
    /// The expected result is an empty stream
    fn test_parse_empty_source_code() {
        // Arrange
        let source_code: Vec<char> = vec![];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![];
        assert_eq!(stream_creator.processed, expected);
    }

    #[test]
    /// # Test StreamCreator with multiple consecutive newlines
    /// The source code is `['a', '\n', '\n', 'b']`
    /// The expected result includes proper line increments across consecutive newlines
    fn test_parse_source_code_with_multiple_newlines() {
        // Arrange
        let source_code: Vec<char> = vec!['a', '\n', '\n', 'b'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('\n', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('\n', Line::new(2).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('b', Line::new(3).unwrap(), Position::new(1).unwrap()),
        ];
        assert_eq!(stream_creator.processed, expected);
    }

    #[test]
    /// # Test StreamCreator initialized with explicit line and position
    /// The source code is `['a', 'b']` with starting line 3, position 5
    fn test_custom_initialization() {
        // Arrange
        let source_code: Vec<char> = vec!['a', 'b'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], Some(3), Some(5));

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(3).unwrap(), Position::new(5).unwrap()),
            SourceCodeCharacter::new('b', Line::new(3).unwrap(), Position::new(6).unwrap()),
        ];
        assert_eq!(stream_creator.processed, expected);
    }

    #[test]
    /// # Test get_processed method
    /// Verify that get_processed returns the correct vector of SourceCodeCharacters
    fn test_get_processed() {
        // Arrange
        let source_code: Vec<char> = vec!['a', 'b'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);
        let stream_creator = stream_creator.generate();

        // Act
        let processed = stream_creator.get_processed();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('b', Line::new(1).unwrap(), Position::new(2).unwrap()),
        ];
        assert_eq!(processed, expected);
    }

    #[test]
    /// # Test StreamCreator with whitespace characters
    /// The source code includes space and tab characters
    fn test_parse_source_code_with_whitespace() {
        // Arrange
        let source_code: Vec<char> = vec!['a', ' ', '\t', 'b'];
        let stream_creator = SourceStreamGenerator::new(source_code, vec![], None, None);

        // Act
        let stream_creator: SourceStreamGenerator = stream_creator.generate();

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('a', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('\t', Line::new(1).unwrap(), Position::new(3).unwrap()),
            SourceCodeCharacter::new('b', Line::new(1).unwrap(), Position::new(4).unwrap()),
        ];
        assert_eq!(stream_creator.processed, expected);
    }
}
