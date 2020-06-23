use codeframe::{capture_codeframe, Color};

use k9::*;

macro_rules! get_codeframe {
    () => {{
        let codeframe = capture_codeframe!(Color::Blue);
        codeframe
    }};
}

macro_rules! get_codeframe_with_default_color {
    () => {{
        let codeframe = capture_codeframe!();
        codeframe
    }};
}

#[test]
fn with_color() {
    super::setup_test_env();
    let codeframe = get_codeframe!();
    if let Some(_codeframe) = &codeframe {}
    assert_matches_inline_snapshot!(
        format!("\n{}", &codeframe.expect("must be present")),
         "
[2m20 | fn with_color() {[0m
[2m21 |     super::setup_test_env();[0m
[34m22 |     let codeframe = get_codeframe!();[0m
[2m23 |     if let Some(_codeframe) = &codeframe {}[0m
[2m24 |     assert_matches_inline_snapshot!([0m
"
    );
}

#[test]
fn without_color() {
    super::setup_test_env();
    let codeframe = get_codeframe_with_default_color!();
    if let Some(_codeframe) = &codeframe {}
    assert_matches_inline_snapshot!(
        format!("\n{}", &codeframe.expect("must be present")),
         "
[2m37 | fn without_color() {[0m
[2m38 |     super::setup_test_env();[0m
[31m39 |     let codeframe = get_codeframe_with_default_color!();[0m
[2m40 |     if let Some(_codeframe) = &codeframe {}[0m
[2m41 |     assert_matches_inline_snapshot!([0m
"
    );
}
