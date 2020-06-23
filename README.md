# Codeframe

[![Crates.io][crates-badge]][crates-url]
![Rust CI](https://github.com/thewebdevel/codeframe/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/codeframe.svg
[crates-url]: https://crates.io/crates/codeframe

Capture pretty code-frames.

```rust
use codeframe::{Color, Codeframe};

let raw_lines = "macro_rules! test_simple_style {
    ($string:expr, $style:ident) => {
        #[test]
        fn $style() {
            assert_eq!(
                s.$style().to_string(),
                ansi_term::Style::new().$style().paint(s).to_string()
            )
        }
    };
}";
let codeframe = Codeframe::new(raw_lines, 5).set_color(Color::Red).capture();
```

![Imgur](https://i.imgur.com/vJzKeCr.png)

### Colors Supported

- Black
- Red
- Green
- Yellow
- Blue
- Magenta or (Purple)
- Cyan
- White

### Features

- [x] line
- [x] raw lines
- [x] color
- [x] capture_codeframe!()
- [ ] file path
- [ ] with column (Highlight Column)
- [ ] with line_above
- [ ] with line_below
