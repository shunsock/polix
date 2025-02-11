use core::source_code::{Line, Position};

#[derive(Clone, Debug)]
pub(crate) struct Buffer {
    pub(crate) text: String,
    pub(crate) current_line: Line,
    pub(crate) current_position: Position,
    pub(crate) start_line: Line,
    pub(crate) start_position: Position,
}

impl Buffer {
    pub(crate) fn new() -> Buffer {
        Buffer {
            text: String::new(),
            current_line: Line::new(1).unwrap(),
            current_position: Position::new(1).unwrap(),
            start_line: Line::new(1).unwrap(),
            start_position: Position::new(1).unwrap(),
        }
    }
}
