use core::source_code::SourceCodeCharacter;

#[derive(Debug, PartialEq, Clone)]
/// # A struct that removes comments from the source code
///
/// ## Example:
/// before: "rebind x: int    = 0;"
/// after:  "rebind x: int = 0;"
pub struct MultipleSpaceRemover {
    source: Vec<SourceCodeCharacter>,
    processed: Vec<SourceCodeCharacter>,
}

impl MultipleSpaceRemover {
    pub fn new(
        source: Vec<SourceCodeCharacter>,
        processed: Vec<SourceCodeCharacter>,
    ) -> MultipleSpaceRemover {
        MultipleSpaceRemover { source, processed }
    }

    pub fn remove(&mut self, reading_space: bool) -> MultipleSpaceRemover {
        match self.source.len() {
            0 => MultipleSpaceRemover::new(vec![], self.processed.clone()),
            1 => {
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();
                let first_char: SourceCodeCharacter = self.source[0].clone();

                if !first_char.character.is_whitespace() && first_char.character != '\n' {
                    processed.push(self.source[0].clone());
                }

                MultipleSpaceRemover::new(vec![], processed)
            }
            _ => {
                let mut source: Vec<SourceCodeCharacter> = self.source.clone();
                let mut processed: Vec<SourceCodeCharacter> = self.processed.clone();

                let first_char: SourceCodeCharacter = source.remove(0);
                if reading_space {
                    return if first_char.character.is_whitespace() || first_char.character == '\n' {
                        MultipleSpaceRemover::new(source, processed).remove(true)
                    } else {
                        processed.push(first_char.clone());
                        MultipleSpaceRemover::new(source, processed).remove(false)
                    };
                }

                if first_char.character.is_whitespace() || first_char.character == '\n' {
                    let space_char = SourceCodeCharacter::new(
                        ' ',
                        first_char.line.clone(),
                        first_char.position.clone(),
                    );
                    processed.push(space_char);
                    MultipleSpaceRemover::new(source, processed).remove(true)
                } else {
                    processed.push(first_char.clone());
                    MultipleSpaceRemover::new(source, processed).remove(false)
                }
            }
        }
    }

    pub fn get_processed(&self) -> Vec<SourceCodeCharacter> {
        self.processed.clone()
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
    /// # Test remover removes multiple spaces
    ///
    /// ## Test Case:
    /// - input source code is: 1   ;
    /// - expected: 1 ;
    fn test_remove_multiple_spaces() {
        // Arrange
        let source_text = String::from("1   ;");
        let source_code: Vec<SourceCodeCharacter> =
            create_source_code_char_factory(source_text.clone());

        // Act
        let remover = MultipleSpaceRemover::new(source_code, vec![]).remove(false);

        // Assert
        let expected: Vec<SourceCodeCharacter> =
            create_source_code_char_factory(String::from("1 ;"));
        assert_eq!(
            remover
                .processed
                .iter()
                .map(|c| (c.character, c.line.number))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|c| (c.character, c.line.number))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    /// # Test remover removes multiple spaces with newline
    ///
    /// ## Test Case:
    /// - input source code is: 1 \n  ;
    /// - expected: 1 ;
    fn test_remove_multiple_spaces_with_newline() {
        // Arrange
        let source_text = String::from("1 \n  ;");
        let source_code: Vec<SourceCodeCharacter> =
            create_source_code_char_factory(source_text.clone());

        // Act
        let remover = MultipleSpaceRemover::new(source_code, vec![]).remove(false);

        // Assert
        let expected: Vec<SourceCodeCharacter> =
            create_source_code_char_factory(String::from("1 ;"));
        assert_eq!(
            remover
                .processed
                .iter()
                .map(|c| (c.character, c.line.number))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|c| (c.character, c.line.number))
                .collect::<Vec<_>>()
        );
    }
}
