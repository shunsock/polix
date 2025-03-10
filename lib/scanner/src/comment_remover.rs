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
        let newline =
            SourceCodeCharacter::new('\n', Line::new(1).unwrap(), Position::new(1).unwrap());
        let mut v: Vec<SourceCodeCharacter> = source.clone();
        v.push(newline.clone());
        v.push(newline);

        CommentRemover {
            source: v,
            processed,
        }
    }

    pub fn remove(&mut self, reading_comment: bool) -> CommentRemover {
        match self.source.len() == 1 {
            true => {
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                processed.push(self.source.pop().unwrap());
                CommentRemover::new(vec![], processed)
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
