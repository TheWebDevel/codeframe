use colored::*;

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Purple,
    Red,
    White,
    Yellow,
}

pub fn color_line(line: &str, color: Color) -> String {
    match color {
        Color::Black => line.black().to_string(),
        Color::Red => line.red().to_string(),
        Color::Green => line.green().to_string(),
        Color::Yellow => line.yellow().to_string(),
        Color::Blue => line.blue().to_string(),
        Color::Magenta => line.magenta().to_string(),
        Color::Purple => line.purple().to_string(),
        Color::Cyan => line.cyan().to_string(),
        Color::White => line.white().to_string(),
    }
}
