//! Pretty Capture codeframes
//!
//! Ex:
//! let raw_lines = "let a: i64 = 12;";
//! let codeframe = Codeframe::new(raw_lines, 1).set_color("red").capture();
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
    color: crate::Color,
    line: i64,
    raw_lines: &'a str,
}

impl<'a> Codeframe<'a> {
    pub fn new(raw_lines: &'a str, line: i64) -> Codeframe<'a> {
        Codeframe {
            color: crate::Color::Red,
            line,
            raw_lines,
        }
    }

    pub fn set_color(mut self, color: crate::Color) -> Self {
        self.color = color;
        self
    }

    pub fn capture(self) -> Option<String> {
        let vec_lines = self.raw_lines.split('\n').collect();
        capture::capture_code_frame(vec_lines, self.line, self.color)
    }
}
