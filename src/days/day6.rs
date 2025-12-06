pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
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
            return None;
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
                return None;
            }
        };
        for row in 0..rows {
            match operations[col].as_str() {
                "*" => col_answer *= numbers[row][col],
                "+" => col_answer += numbers[row][col],
                _ => {
                    eprintln!("Error unsupported operation: '{}'", operations[col]);
                    return None;
                }
            };
        }
        answer += col_answer;
    }

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut lines: Vec<String> = file_lines.collect();

    let operations: String = match lines.pop() {
        Some(line) => line,
        None => {
            eprintln!("Error no lines provided...");
            return None;
        }
    };

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    if max_len == 0 {
        eprintln!("Empty input!");
        return None;
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

    Some(answer)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day6";

    #[test]
    fn test_day6_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 4277556;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day6_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 7644505810277;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day6_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 3263827;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day6_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 12841228084455;
        assert_eq!(expected, actual);
    }
}
