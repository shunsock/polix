use core::source_code::SourceCodeCharacter;
use core::source_code::Line;
use core::source_code::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct StreamCreator {
    source_code_chars: Vec<char>,
    stream: Vec<SourceCodeCharacter>,
    line: u32,
    position: u32,
}

impl StreamCreator {
    pub fn new(
        source_code: Vec<char>,
        stream: Vec<SourceCodeCharacter>,
        index: Option<u32>,
        position: Option<u32>
    ) -> StreamCreator {
        match (index, position) {
            (Some(i), Some(p)) => {
                StreamCreator {
                    source_code_chars: source_code,
                    stream,
                    line: i,
                    position: p,
                }
            }
            _ => {
                StreamCreator {
                    source_code_chars: source_code,
                    stream,
                    line: 1,
                    position: 1,
                }
            }
        }
    }

    pub fn create_stream(&self) {
        match self.source_code_chars.len() {
            0 => {}
            _ => {
                // Get the first character from the source code
                // This unwrap is ok, because we are sure that the vec is not empty
                let mut rest_chars: Vec<char> = self.source_code_chars.clone();
                let first_char: char = rest_chars.pop().unwrap();

                // create a new SourceCodeCharacter
                // These unwraps are ok, because we are sure that the line and position are greater than 0
                let source_code_char = SourceCodeCharacter::new(
                    first_char,
                    Line::new(self.line).unwrap(),
                    Position::new(self.position).unwrap(),
                );

                // push the new SourceCodeCharacter to the stream
                let mut updated_stream: Vec<SourceCodeCharacter> = self.stream.clone();
                updated_stream.push(source_code_char);

                // if the first character is a newline character, increment the line and reset the position
                if first_char == '\n' {
                    return StreamCreator::new(
                        rest_chars,
                        updated_stream,
                        Some(self.line + 1),
                        Some(1),
                    ).create_stream();
                }

                // create a new StreamCreator with the rest of the source code
                StreamCreator::new(
                    rest_chars,
                    updated_stream,
                    Some(self.line),
                    Some(self.position + 1),
                ).create_stream();
            }
        }
    }
}
