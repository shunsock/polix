use core::source_code::Line;
use core::source_code::Position;

/// ## Reporter
/// `Reporter` is a struct that is responsible for reporting errors and warnings to the user.
///
/// ### Arguments
/// - `src` - A string that contains the source code.
///
/// ### Methods
/// - `new` - Creates a new instance of `Reporter`.
/// - `report` - Reports an error or warning to the user.
pub struct Reporter {
    source_code_lines: Vec<String>,
}

impl Reporter {
    pub fn new(src: String) -> Self {
        let mut source_code_lines: Vec<String> = Vec::new();
        for line in src.lines() {
            source_code_lines.push(line.to_string());
        }
        Self { source_code_lines }
    }

    pub fn report(&self, line: Line, position: Position, message: &str) {
        let target_line_content: String = self.get_line_content(line);
        let caret: String = self.generate_caret(position);
        eprintln!("{}: {}", target_line_content, message);
        eprintln!("{}", caret);
    }

    fn get_line_content(&self, line: Line) -> String {
        match self.source_code_lines.get((line.number.value - 1) as usize) {
            Some(content) => content.to_string(),
            None => panic!(
                "[Fetal] Line number is out of range. Expected: 0..{}, Found: {}. Please report this issue to the developers.",
                self.source_code_lines.len(),
                line.number.value,
            ),
        }
    }

    fn generate_caret(&self, position: Position) -> String {
        let position: u32 = position.number.value;
        let mut caret: String = String::new();
        for _ in 0..position {
            caret.push(' ');
        }
        caret.push('^');
        caret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::positive_integer::PositiveInteger32;
    use core::source_code::Line;
    use core::source_code::Position;

    /// ## Test get_line_content return the content of the line
    ///
    /// Expectation: "second line"
    #[test]
    fn test_get_line_content_valid() {
        // Expectation
        let expected: &str = "second line";

        // Given
        let src: String = "first line\nsecond line\nthird line".to_string();
        let reporter: Reporter = Reporter::new(src);
        let line: Line = Line::new(2).unwrap();

        // When
        let content: String = reporter.get_line_content(line);

        // Then
        assert_eq!(content, expected);
    }

    /// ## Test get_line_content fails when the line number is out of range
    ///
    /// Expectation: panic with message "[Fetal] Line number is out of range"
    #[test]
    #[should_panic(expected = "[Fetal] Line number is out of range")]
    fn test_get_line_content_invalid() {
        // Given
        let src: String = "only one line".to_string();
        let reporter: Reporter = Reporter::new(src);
        let line: Line = Line::new(100).unwrap();

        // When
        reporter.get_line_content(line);
    }

    /// ## Test generate_caret return a caret with the position
    ///
    /// input:  "dummy"
    /// output: "   ^"
    #[test]
    fn test_generate_caret() {
        // Expectation
        // dummy
        //    ^
        let expected: &str = "    ^";

        // Given
        let reporter: Reporter = Reporter::new("dummy".to_string());
        let position: Position = Position::new(4).unwrap();

        // When
        let caret: String = reporter.generate_caret(position);

        // Then
        assert_eq!(caret, expected);
    }
}
