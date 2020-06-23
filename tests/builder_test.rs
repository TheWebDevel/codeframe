use codeframe::Codeframe;
extern crate k9;

#[test]
fn simple_capture() {
    setup_test_env();
    let raw_lines = "let a: i64 = 12;";
    let codeframe = Codeframe::new(raw_lines, 1).set_color("red").capture();
    k9::assert_equal!(
        Some("\u{1b}[31m1 | let a: i64 = 12;\u{1b}[0m\n".to_owned()),
        codeframe
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
    let codeframe = Codeframe::new(raw_lines, 5).set_color("Red").capture();

    k9::assert_equal!(
        Some("\u{1b}[2m3 |         #[test]\u{1b}[0m\n\u{1b}[2m4 |         fn $style() {\u{1b}[0m\n\u{1b}[31m5 |             assert_eq!(\u{1b}[0m\n\u{1b}[31m6 |                 s.$style().to_string(),\u{1b}[0m\n\u{1b}[31m7 |                 ansi_term::Style::new().$style().paint(s).to_string()\u{1b}[0m\n\u{1b}[31m8 |             )\u{1b}[0m\n".to_owned()),
        codeframe
    );
}

#[test]
fn out_of_bound_line_number() {
    setup_test_env();
    let raw_lines = "let a: i64 = 12;";
    let codeframe = Codeframe::new(raw_lines, 2).set_color("black").capture();
    k9::assert_equal!(
        Some("\u{1b}[2m1 | let a: i64 = 12;\u{1b}[0m\n".to_owned()),
        codeframe
    );
}

fn setup_test_env() {
    colored::control::set_override(true);
}
