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
    pub fn new(source: Vec<SourceCodeCharacter>, processed: Vec<SourceCodeCharacter>) -> CommentRemover {
        CommentRemover {
            source,
            processed,
        }
    }

    pub fn remove(&mut self, reading_comment: bool) -> CommentRemover {
        match self.source.len() == 1 {
            true => {
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                processed.push(self.source.pop().unwrap());
                CommentRemover::new(vec![], processed)
            },
            false => {
                let mut source = self.source.clone();
                let first_char = source.remove(0);
                let second_char = source.remove(0);

                // processing the first character
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                processed.push(first_char.clone());

                // if reading a comment and the first character is not newline, continue
                if reading_comment && first_char.character != '\n'  {
                    return CommentRemover::new(source, processed).remove(true);
                };
                // if reading a comment and the first character is newline, stop reading the comment
                if reading_comment && first_char.character == '\n'  {
                    return CommentRemover::new(source, processed).remove(false);
                };

                // processing the second character
                processed.push(second_char.clone());

                // if the first character is a slash, check if the next character is a slash
                let is_comment_out = first_char.character == '/' && second_char.character == '/';
                if is_comment_out {
                    return CommentRemover::new(source, processed).remove(true);
                };

                CommentRemover::new(source, processed).remove(false)
            }
        }
    }
}