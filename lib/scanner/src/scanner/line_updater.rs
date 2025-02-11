use core::source_code::Line;

pub(super) fn update(c: char, l: Line) -> Line {
    match c {
        '\n' => l.increment(),
        _ => l,
    }
}
