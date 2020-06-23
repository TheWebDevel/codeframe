use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[macro_export]
macro_rules! capture_codeframe {
    () => {{
        let line = line!() as i64;
        let file = file!();
        let lines = $crate::codeframe_macro::read_file(file);
        use $crate::capture::capture_codeframe;
        let codeframe = capture_codeframe(lines, line, "red");
        codeframe
    }};
    ($color:expr) => {{
        let line = line!() as i64;
        let file = file!();
        let lines = $crate::codeframe_macro::read_file(file);
        use $crate::capture::capture_codeframe;
        let codeframe = capture_codeframe(lines, line, $color);
        codeframe
    }};
}
