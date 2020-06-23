use codeframe::{Codeframe, Color};

use k9::*;

#[test]
fn simple_capture() {
    setup_test_env();
    let raw_lines = "let a: i64 = 12;";
    let codeframe = Codeframe::new(raw_lines, 1).set_color(Color::Red).capture();

    assert_matches_inline_snapshot!(
        format!("\n{}", codeframe.expect("must be present")),
        "
[31m1 | let a: i64 = 12;[0m
"
    );

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

    assert_matches_inline_snapshot!(
        format!("\n{}", codeframe.expect("must be present")),
        "
[2m3 |         #[test][0m
[2m4 |         fn $style() {[0m
[31m5 |             assert_eq!([0m
[31m6 |                 s.$style().to_string(),[0m
[31m7 |                 ansi_term::Style::new().$style().paint(s).to_string()[0m
[31m8 |             )[0m
"
    );
}

#[test]
fn out_of_bound_line_number() {
    setup_test_env();
    let raw_lines = "let a: i64 = 12;";
    let codeframe = Codeframe::new(raw_lines, 2)
        .set_color(Color::Black)
        .capture();
    assert_matches_inline_snapshot!(
        format!("\n{}", codeframe.expect("must be present")),
        "
[2m1 | let a: i64 = 12;[0m
"
    );
}

fn setup_test_env() {
    colored::control::set_override(true);
}
