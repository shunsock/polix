//! # Polix Reporter
//!
//! `reporter` is a crate for the Polix language that provides error reporting functionality.
//! It helps display clear, user-friendly error messages with proper source location information.
//!
//! ## Features
//!
//! - Line and column positioning of errors
//! - Visual indication of error location with carets
//! - Clear error messages for better debugging
//!
//! ## Example
//!
//! ```
//! use core::source_code::{Line, Position};
//! use reporter::Reporter;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let source_code = "let x: int = \"hello\";".to_string();
//! let reporter = Reporter::new(source_code);
//!
//! // Report an error at line 1, position 12
//! reporter.report(
//!     Line::new(1).unwrap(),
//!     Position::new(12).unwrap(),
//!     "Type mismatch: expected int, found string"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! This will produce output like:
//!
//! ```text
//! ERROR: position: 13, line: 1, Type mismatch: expected int, found string
//! let x: int = "hello";
//!             ^
//! ```

use core::source_code::Line;
use core::source_code::Position;

/// A struct that is responsible for reporting errors and warnings to the user.
///
/// The Reporter takes a source code string and provides methods to report errors
/// with line and position information, showing the problematic code and indicating
/// the exact position with a caret (^).
///
/// # Examples
///
/// ```
/// use core::source_code::{Line, Position};
/// use reporter::Reporter;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let src = "let x = 5;\nlet y = \"hello\";\n".to_string();
/// let reporter = Reporter::new(src);
///
/// // Report an error on line 2, position 8
/// reporter.report(
///     Line::new(2).unwrap(),
///     Position::new(8).unwrap(),
///     "String literals must use double quotes"
/// );
/// # Ok(())
/// # }
/// ```
pub struct Reporter {
    source_code_lines: Vec<String>,
}

impl Reporter {
    /// Creates a new instance of the Reporter.
    ///
    /// # Arguments
    ///
    /// * `src` - A string containing the source code to be analyzed
    ///
    /// # Returns
    ///
    /// A new Reporter instance
    ///
    /// # Examples
    ///
    /// ```
    /// use reporter::Reporter;
    ///
    /// let src = "let x = 5;".to_string();
    /// let reporter = Reporter::new(src);
    /// ```
    pub fn new(src: String) -> Self {
        let mut source_code_lines: Vec<String> = Vec::new();
        for line in src.lines() {
            source_code_lines.push(line.to_string());
        }
        Self { source_code_lines }
    }

    /// Reports an error to the user with line, position and message information.
    ///
    /// This method prints:
    /// 1. An error message with position and line information
    /// 2. The line of code where the error occurred
    /// 3. A caret (^) pointing to the exact position of the error
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the error occurred
    /// * `position` - The position (column) where the error occurred
    /// * `message` - The error message to display
    ///
    /// # Examples
    ///
    /// ```
    /// use core::source_code::{Line, Position};
    /// use reporter::Reporter;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let src = "let x = 5;".to_string();
    /// let reporter = Reporter::new(src);
    ///
    /// reporter.report(
    ///     Line::new(1).unwrap(),
    ///     Position::new(4).unwrap(),
    ///     "Variable name cannot be a single letter"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn report(&self, line: Line, position: Position, message: &str) {
        let target_line_content: String = self.get_line_content(line);
        let caret: String = self.generate_caret(position);
        eprintln!(
            "ERROR: position: {}, line: {}, {}",
            position.number.get(),
            line.number.get(),
            message
        );
        eprintln!("{}", target_line_content);
        eprintln!("{}", caret);
    }

    /// Gets the content of a specific line from the source code.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to retrieve
    ///
    /// # Returns
    ///
    /// The content of the specified line as a String
    ///
    /// # Panics
    ///
    /// If the line number is out of range for the source code
    fn get_line_content(&self, line: Line) -> String {
        match self.source_code_lines.get((line.number.get() - 1) as usize) {
            Some(content) => content.to_string(),
            None => panic!(
                "[Fetal] Line number is out of range. Expected: 0..{}, Found: {}. Please report this issue to the developers.",
                self.source_code_lines.len(),
                line.number.get(),
            ),
        }
    }

    /// Generates a caret string to point to the error position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position (column) where the error occurred
    ///
    /// # Returns
    ///
    /// A string with spaces followed by a caret (^) at the error position
    fn generate_caret(&self, position: Position) -> String {
        let position: u32 = position.number.get();
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
