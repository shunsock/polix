# Polix Reporter

The Reporter crate is part of the Polix language toolchain. It is responsible for reporting errors and warnings to the user in a clear and informative way.

## Features

- Line and column positioning of errors
- Visual indication of error location with carets
- Clear error messages for better debugging

## Usage

```rust
use core::source_code::{Line, Position};
use reporter::Reporter;

fn main() {
    let source_code = "let x: int = \"hello\";".to_string();
    let reporter = Reporter::new(source_code);

    // Report an error at line 1, position 12
    reporter.report(
        Line::new(1).unwrap(),
        Position::new(12).unwrap(),
        "Type mismatch: expected int, found string"
    );
}
```

## Output Example

```
ERROR: position: 13, line: 1, Type mismatch: expected int, found string
let x: int = "hello";
            ^
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
