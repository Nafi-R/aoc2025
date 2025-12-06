use utils::get_day_lines;

pub fn part1() {
    let file_lines = match get_day_lines("day6") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

    let mut lines: Vec<String> = file_lines.collect();

    let operations: Vec<String> = match lines.pop() {
        Some(line) => line
            .chars()
            .filter_map(|x| match x {
                ' ' => None,
                _ => Some(x.to_string()),
            })
            .collect(),
        None => {
            eprintln!("Error no lines provided...");
            return;
        }
    };

    let numbers: Vec<Vec<i64>> = lines
        .iter()
        .map(|row| {
            row.split(" ")
                .filter_map(|number| match number.parse::<i64>() {
                    Ok(number) => Some(number),
                    Err(_) => {
                        return None;
                    }
                })
                .collect()
        })
        .collect();

    let mut answer: i64 = 0;
    let columns = operations.len();
    let rows = numbers.len();

    for col in 0..columns {
        let mut col_answer = match operations[col].as_str() {
            "*" => 1,
            "+" => 0,
            _ => {
                eprintln!("Error unsupported operation: '{}'", operations[col]);
                return;
            }
        };
        for row in 0..rows {
            match operations[col].as_str() {
                "*" => col_answer *= numbers[row][col],
                "+" => col_answer += numbers[row][col],
                _ => {
                    eprintln!("Error unsupported operation: '{}'", operations[col]);
                    return;
                }
            };
        }
        answer += col_answer;
    }

    println!("Day6 Part 1 answer: {:?}", answer);
}

pub fn part2() {
    let file_lines = match get_day_lines("day6") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

    let mut lines: Vec<String> = file_lines.collect();

    let operations: String = match lines.pop() {
        Some(line) => line,
        None => {
            eprintln!("Error no lines provided...");
            return;
        }
    };

    // The key change for Part 2 is to process the input column-by-column,
    // where each column forms a number. To do this reliably, we must
    // parse the input by character position, not by splitting on spaces.

    // 1. Determine the width of the input (all rows must be padded to the same width)
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    if max_len == 0 {
        eprintln!("Empty input!");
        return;
    }

    let n_rows = lines.len();
    let mut numbers: Vec<i64> = Vec::new();
    let mut answer: i64 = 0;
    let mut last_op: String = String::from("+");

    for char_col in 0..max_len {
        match operations.chars().nth(char_col) {
            Some(op) => {
                match &op {
                    '+' | '*' => {
                        let col_answer = match last_op.as_str() {
                            "*" => numbers.iter().product::<i64>(),
                            "+" => numbers.iter().sum::<i64>(),
                            _ => 0,
                        };
                        answer += col_answer;
                        numbers.clear();
                        last_op = op.to_string();
                    }
                    _ => (),
                };
            }
            _ => (),
        };

        let mut digits: Vec<char> = Vec::new();
        for row in 0..n_rows {
            let curr_line = lines.get(row).unwrap();
            match curr_line.chars().nth(char_col) {
                Some(c) => match c {
                    ' ' => {}
                    _ => {
                        digits.push(c);
                    }
                },
                None => (),
            };
        }
        let number_str: String = digits.iter().collect();
        let number = match number_str.parse::<i64>() {
            Ok(number) => number,
            Err(_) => {
                continue;
            }
        };

        numbers.push(number);
    }

    answer += match last_op.as_str() {
        "*" => numbers.iter().product::<i64>(),
        "+" => numbers.iter().sum::<i64>(),
        _ => 0,
    };

    println!("Day6 Part 2 answer: {:?}", answer);
}
