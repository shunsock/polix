use core::source_code::Line;
use core::source_code::Position;
use core::source_code::SourceCodeCharacter;

#[derive(Debug, PartialEq, Clone)]
pub struct SourceStreamGenerator {
    rest_source_code: Vec<char>,
    processed: Vec<SourceCodeCharacter>,
    line: u32,
    position: u32,
}

impl SourceStreamGenerator {
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
}
