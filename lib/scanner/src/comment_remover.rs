use core::source_code::Line;
use core::source_code::Position;
use core::source_code::SourceCodeCharacter;

#[derive(Debug, PartialEq, Clone)]
/// A struct that removes comments from the source code
///
/// Example:
/// before: rebind x: int = 0; // this is a comment
/// after:  rebind x: int = 0;
pub struct CommentRemover {
    source: Vec<SourceCodeCharacter>,
    processed: Vec<SourceCodeCharacter>,
}

impl CommentRemover {
    pub fn new(
        source: Vec<SourceCodeCharacter>,
        processed: Vec<SourceCodeCharacter>,
    ) -> CommentRemover {
        CommentRemover {
            source,
            processed,
        }
    }

    pub fn remove(&mut self, reading_comment: bool) -> CommentRemover {
        match self.source.len() {
            0 => {
                CommentRemover::new(vec![], self.processed.clone())
            },
            1 => {
                let mut processed = self.processed.clone();

                // If we're not in a comment, include the last character
                if !reading_comment {
                    processed.push(self.source[0].clone());
                }

                CommentRemover::new(vec![], processed)
            },
            _ => {
                let mut source: Vec<SourceCodeCharacter> = self.source.clone();
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();

                if reading_comment {
                    let first_char: SourceCodeCharacter = source.remove(0);

                    // newline is the end of the comment
                    if first_char.character == '\n' {
                        processed.push(first_char.clone());
                        return CommentRemover::new(source, processed).remove(false);
                    }

                    // skip the first character
                    return CommentRemover::new(source, processed).remove(true);
                }

                // Not in comment mode - check if this could be a comment start
                if source[0].character == '/' && source.len() >= 2 && source[1].character == '/' {
                    // Found a comment start - remove both slashes
                    source.remove(0); // Remove first slash
                    source.remove(0); // Remove second slash

                    // Enter comment mode without adding slashes to processed
                    return CommentRemover::new(source, processed).remove(true);
                }

                // Not a comment or comment start - add the first character to processed
                let first_char: SourceCodeCharacter = source.remove(0);
                processed.push(first_char);

                // Continue processing
                CommentRemover::new(source, processed).remove(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a vector of SourceCodeCharacter from a string
    /// Each character will have a Line number of 1 and sequential Position numbers
    fn create_source_code_char_factory(text: String) -> Vec<SourceCodeCharacter> {
        let mut result = Vec::with_capacity(text.len());

        for (i, ch) in text.chars().enumerate() {
            let position = i + 1; // Position is 1-indexed
            result.push(SourceCodeCharacter::new(
                ch,
                Line::new(1).unwrap(),
                Position::new(position as u32).unwrap()
            ));
        }

        result
    }

    #[test]
    /// # Test remover pass without comment
    ///
    /// The source code is: rebind x: int = 0;
    /// expected: rebind x: int = 0;
    fn test_remover_pass_without_comment() {
        // Arrange
        let source_text = String::from("rebind x: int = 0;");
        let source_code = create_source_code_char_factory(source_text.clone());

        // Act
        let remover = CommentRemover::new(source_code, vec![]).remove(false);

        // Assert
        let expected = create_source_code_char_factory(source_text);
        assert_eq!(remover.processed, expected);
    }

    #[test]
    /// # Test remover pass without comment
    ///
    /// The source code is: rebind x: int = 0; // this is a comment
    /// expected: rebind x: int = 0;
    fn test_remover_pass_with_comment() {
        // Arrange
        let source_text = String::from("rebind x: int = 0; // this is a comment");
        let source_code = create_source_code_char_factory(source_text.clone());

        // Act
        let remover = CommentRemover::new(source_code, vec![]).remove(false);

        // Assert
        let expected = create_source_code_char_factory("rebind x: int = 0; ".to_string());
        assert_eq!(remover.processed, expected);
    }
}