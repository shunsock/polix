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

    #[test]
    /// # Test remover pass without comment
    ///
    /// The source code is: rebind x: int = 0;
    /// expected: rebind x: int = 0;
    fn test_remover_pass_without_comment() {
        // Arrange
        let source_code: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('r', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('e', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('b', Line::new(1).unwrap(), Position::new(3).unwrap()),
            SourceCodeCharacter::new('i', Line::new(1).unwrap(), Position::new(4).unwrap()),
            SourceCodeCharacter::new('n', Line::new(1).unwrap(), Position::new(5).unwrap()),
            SourceCodeCharacter::new('d', Line::new(1).unwrap(), Position::new(6).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(7).unwrap()),
            SourceCodeCharacter::new('x', Line::new(1).unwrap(), Position::new(8).unwrap()),
            SourceCodeCharacter::new(':', Line::new(1).unwrap(), Position::new(9).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(10).unwrap()),
            SourceCodeCharacter::new('i', Line::new(1).unwrap(), Position::new(11).unwrap()),
            SourceCodeCharacter::new('n', Line::new(1).unwrap(), Position::new(12).unwrap()),
            SourceCodeCharacter::new('t', Line::new(1).unwrap(), Position::new(13).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(14).unwrap()),
            SourceCodeCharacter::new('=', Line::new(1).unwrap(), Position::new(15).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(16).unwrap()),
            SourceCodeCharacter::new('0', Line::new(1).unwrap(), Position::new(17).unwrap()),
            SourceCodeCharacter::new(';', Line::new(1).unwrap(), Position::new(18).unwrap()),
        ];

        // Act
        let remover = CommentRemover::new(source_code, vec![]).remove(false);

        // Assert
        let expected: Vec<SourceCodeCharacter> = vec![
            SourceCodeCharacter::new('r', Line::new(1).unwrap(), Position::new(1).unwrap()),
            SourceCodeCharacter::new('e', Line::new(1).unwrap(), Position::new(2).unwrap()),
            SourceCodeCharacter::new('b', Line::new(1).unwrap(), Position::new(3).unwrap()),
            SourceCodeCharacter::new('i', Line::new(1).unwrap(), Position::new(4).unwrap()),
            SourceCodeCharacter::new('n', Line::new(1).unwrap(), Position::new(5).unwrap()),
            SourceCodeCharacter::new('d', Line::new(1).unwrap(), Position::new(6).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(7).unwrap()),
            SourceCodeCharacter::new('x', Line::new(1).unwrap(), Position::new(8).unwrap()),
            SourceCodeCharacter::new(':', Line::new(1).unwrap(), Position::new(9).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(10).unwrap()),
            SourceCodeCharacter::new('i', Line::new(1).unwrap(), Position::new(11).unwrap()),
            SourceCodeCharacter::new('n', Line::new(1).unwrap(), Position::new(12).unwrap()),
            SourceCodeCharacter::new('t', Line::new(1).unwrap(), Position::new(13).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(14).unwrap()),
            SourceCodeCharacter::new('=', Line::new(1).unwrap(), Position::new(15).unwrap()),
            SourceCodeCharacter::new(' ', Line::new(1).unwrap(), Position::new(16).unwrap()),
            SourceCodeCharacter::new('0', Line::new(1).unwrap(), Position::new(17).unwrap()),
            SourceCodeCharacter::new(';', Line::new(1).unwrap(), Position::new(18).unwrap()),
        ];

        assert_eq!(remover.processed, expected);
    }
}