use colored::*;

pub fn color_line(line: &str, color: &str) -> String {
    let streamlined_color = &color.to_lowercase()[..];
    match streamlined_color {
        "black" => line.black().to_string(),
        "red" => line.red().to_string(),
        "green" => line.green().to_string(),
        "yellow" => line.yellow().to_string(),
        "blue" => line.blue().to_string(),
        "magenta" => line.magenta().to_string(),
        "purple" => line.purple().to_string(),
        "cyan" => line.cyan().to_string(),
        "white" => line.white().to_string(),
        _ => line.red().to_string(),
    }
}
