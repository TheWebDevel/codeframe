# Codeframe

[![Crates.io][crates-badge]][crates-url]
![Rust CI](https://github.com/thewebdevel/codeframe/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/codeframe.svg
[crates-url]: https://crates.io/crates/codeframe

Capture pretty code-frames.

## capture_codeframe!()

`capture_codeframe!()` makes use of [line!](https://doc.rust-lang.org/1.2.0/std/macro.line!.html) and [file!](https://doc.rust-lang.org/1.2.0/std/macro.file!.html) to capture the codeframe from the place it was originally invoked.

### Example

Imagine having a macro `assert_equal!(left, right)` that checks whether left is equal to right. We can use `capture_codeframe!()` to get code-frame of `assert_equal!(left, right)` invocation with some context.

It also accepts `Color` argument (`capture_codeframe!(Color::Blue)`) which will default to `Color::Red`.

```Rust
use codeframe::{capture_codeframe, Color};

macro_rules! assert_equal {
    ($left:expr, $right:expr) => {{
        if $left != $right {
            let codeframe = capture_codeframe!(Color::Blue);
            println!("Left does not match Right");
            if let Some(codeframe) = codeframe {
                println!("{}", codeframe)
            }
        } else {
            println!("Left and right are equal");
        }
    }};
}

fn with_context() {
    super::setup_test_env();
    assert_equal!(1, 2);
}

```

Note `let codeframe = capture_codeframe!(Color::Blue);` in the `assert_equal` macro. This captures the code-frame where it was originally invoked. In our case, `assert_equal!(1, 2);`. So the output would be:

![Output](https://i.imgur.com/JwWMP7m.png)

### Usage

- `capture_codeframe!()`
- `capture_codeframe!(Color::Red)`
- `capture_codeframe!(Color::Blue)`

[View currently supported colors](https://github.com/TheWebDevel/codeframe#colors-supported)

## Codeframe with Snippets

You can also capture codeframes with code snippet by making use of the Builder Pattern.

```Rust
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
}".to_owned();
let codeframe = Codeframe::new(raw_lines, 5).set_color(Color::Red).capture();

if let Some(codeframe) = codeframe {
    println!("{}", codeframe)
}
```

The Builder takes raw lines and line number(to highlight) as mandatory arguments. You can additionaly set the highlight color using `set_color(Color::Red)`. This will result with the following:

![Output](https://i.imgur.com/QORF7RQ.png)

### Usage

- `Codeframe::new(raw_lines, 5).capture();`
- `Codeframe::new(raw_lines, 5).set_color(Color::Red).capture();`
- `Codeframe::new(raw_lines, 5).set_color(Color::Blue).capture();`

[View currently supported colors](https://github.com/TheWebDevel/codeframe#colors-supported)

## Return Type

`Option<String>`

## Colors Supported

- Black
- Red
- Green
- Yellow
- Blue
- Magenta or (Purple)
- Cyan
- White

## Features

### Macro

- [x] [capture_codeframe!](https://github.com/TheWebDevel/codeframe#capture_codeframe)

### Builder Pattern

- [x] [raw lines](https://github.com/TheWebDevel/codeframe#codeframe-with-snippets)
- [x] [line to highlight](https://github.com/TheWebDevel/codeframe#codeframe-with-snippets)
- [x] [color](https://github.com/TheWebDevel/codeframe#codeframe-with-snippets)
- [ ] file path
- [ ] with column (Highlight Column)
- [ ] with line_above
- [ ] with line_below
