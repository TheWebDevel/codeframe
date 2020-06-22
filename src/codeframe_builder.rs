//! Pretty Capture codeframes
//!
//! Ex:
//! let raw_lines = "let a: i64 = 12;";
//! let codeframe = Codeframe::new(raw_lines, 1).set_color("red").build();
//!
//!
//! Colors Supported
//! Black
//! Red
//! Green
//! Yellow
//! Blue
//! Magenta or (Purple)
//! Cyan
//! White

use crate::capture;

pub struct Codeframe<'a> {
    color: &'a str,
    line: i64,
    raw_lines: &'a str,
}

impl<'a> Codeframe<'a> {
    pub fn new(raw_lines: &'a str, line: i64) -> Codeframe<'a> {
        Codeframe {
            color: "red",
            line,
            raw_lines,
        }
    }

    pub fn set_color(mut self, color: &'a str) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> Option<String> {
        let vec_lines = self.raw_lines.split('\n').collect();
        capture::capture_code_frame(vec_lines, self.line, self.color)
    }
}
