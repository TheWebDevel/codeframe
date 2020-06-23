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
    let codeframe = get_codeframe!();
    if let Some(codeframe) = &codeframe {
        println!("{}", codeframe);
    }
    assert_matches_inline_snapshot!(
        format!("\n{}", codeframe.expect("must be present")),
        "
[2m12 | #[test][0m
[2m13 | fn with_context() {[0m
[34m14 |     let codeframe = get_codeframe!();[0m
[2m15 |     if let Some(codeframe) = &codeframe {[0m
[2m16 |         println!(\"{}\", codeframe);[0m
"
    );
}
