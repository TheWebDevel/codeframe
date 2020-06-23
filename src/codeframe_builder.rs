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
use crate::Color;

pub struct Codeframe {
    color: Color,
    line: i64,
    raw_lines: String,
}

impl Codeframe {
    pub fn new(raw_lines: String, line: i64) -> Codeframe {
        Codeframe {
            color: Color::Red,
            line,
            raw_lines,
        }
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn capture(self) -> Option<String> {
        let vec_lines = self.raw_lines.split('\n').map(|s| s.to_owned()).collect();
        capture::capture_codeframe(vec_lines, self.line, self.color)
    }
}
