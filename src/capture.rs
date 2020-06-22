use crate::color;
use crate::utils;
use colored::*;

// Constants
const DEFAULT_LINE_ABOVE: i64 = 2;
const DEFAULT_START_LINE: i64 = 1;
const MAX_CODEFRAME_LENGTH: usize = 100;
const MIN_CODEFRAME_LENGTH: usize = 5;

pub fn capture_code_frame(lines: Vec<&str>, line: i64, color: &str) -> Option<String> {
    let mut result = String::new();
    let call_line = line;
    let mut start_loc: i64 = call_line - DEFAULT_LINE_ABOVE;

    if start_loc <= 0 {
        start_loc = DEFAULT_START_LINE
    }

    let file_length = lines.len() as i64;

    let mut position = start_loc;
    let mut start_coloring = false;
    let mut paranthesis_count = 0;
    let code_frame = &lines[(start_loc - 1) as usize..];
    for line in code_frame {
        let mut formatted_line = format!("{} | {}", position, line);
        if position == call_line {
            paranthesis_count += utils::count_paranthesis(line);
            if paranthesis_count != 0 {
                start_coloring = true;
            }
            formatted_line = color::color_line(&formatted_line, color);
        } else if start_coloring {
            paranthesis_count += utils::count_paranthesis(line);
            if paranthesis_count == 0 {
                start_coloring = false;
            }
            formatted_line = color::color_line(&formatted_line, color).to_string();
        } else {
            if utils::count_newlines(&result) >= MIN_CODEFRAME_LENGTH {
                break;
            }
            formatted_line = formatted_line.dimmed().to_string();
            if position > file_length {
                break;
            }
        }
        result.push_str(formatted_line.as_str());
        result.push_str("\n");
        position += 1;

        if utils::count_newlines(&result) >= MAX_CODEFRAME_LENGTH {
            break;
        }
    }

    Some(result)
}
