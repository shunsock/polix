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
        match self.source.len() <= 1 {
            true => {
                if reading_comment {
                    return CommentRemover::new(vec![], self.processed.clone());
                }
                CommentRemover::new(vec![], self.processed.clone())
            }
            false => {
                let mut source: Vec<SourceCodeCharacter> = self.source.clone();
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                let first_char: SourceCodeCharacter = source.remove(0);

                // ignore the first character if it is not a newline character (end of the comment)
                if reading_comment && first_char.character != '\n' {
                    return CommentRemover::new(source, processed).remove(true);
                };

                processed.push(first_char.clone());

                // newline is the end of the comment
                if reading_comment && first_char.character == '\n' {
                    return CommentRemover::new(source, processed).remove(false);
                };

                // processing the second character
                let second_char: SourceCodeCharacter = source.remove(0);
                processed.push(second_char.clone());

                // if the first character is a slash, check if the next character is a slash
                let is_comment_out: bool =
                    first_char.character == '/' && second_char.character == '/';
                if is_comment_out {
                    return CommentRemover::new(source, processed).remove(true);
                };

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
}