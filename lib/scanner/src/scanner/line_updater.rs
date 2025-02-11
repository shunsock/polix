use core::source_code::Line;

pub(super) fn update(c: char, line: Line) -> Line {
    match c {
        '\n' => line.increment(),
        _ => line,
    }
}
