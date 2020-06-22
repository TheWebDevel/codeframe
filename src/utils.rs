pub fn count_paranthesis(line: &str) -> i64 {
    let mut count = 0;

    for bracket in line.chars() {
        let change = match bracket {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        count += change;
    }

    count
}

pub fn count_newlines(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\n')
}
