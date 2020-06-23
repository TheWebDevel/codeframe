use codeframe::{capture_codeframe, Color};

use k9::*;

macro_rules! get_codeframe {
    () => {{
        let codeframe = capture_codeframe!(Color::Blue);
        codeframe
    }};
}

#[test]
fn with_context() {
    super::setup_test_env();
    let codeframe = get_codeframe!();
    if let Some(_codeframe) = &codeframe {}
    assert_matches_inline_snapshot!(
        format!("\n{}", &codeframe.expect("must be present")),
        "
[2m13 | fn with_context() {[0m
[2m14 |     super::setup_test_env();[0m
[34m15 |     let codeframe = get_codeframe!();[0m
[2m16 |     if let Some(_codeframe) = &codeframe {}[0m
[2m17 |     assert_matches_inline_snapshot!([0m
"
    );
}
