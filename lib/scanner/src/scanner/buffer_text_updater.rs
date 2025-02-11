pub(super) fn update(c: char, text: String) -> String {
    match c {
        '\0' | '\t' | '\r' | ' ' | '\n' => text,
        _ => format!("{}{}", text, c),
    }
}
