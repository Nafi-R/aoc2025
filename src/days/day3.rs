pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut answer: i64 = 0;
    for line in file_lines {
        match get_largest_subsequence(&line, 2) {
            Some(value) => {
                answer += value;
            }
            None => {
                eprintln!("Value not found... line:{} ", line);
                return None;
            }
        }
    }

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut answer: i64 = 0;

    for line in file_lines {
        match get_largest_subsequence(&line, 12) {
            Some(value) => {
                answer += value;
            }
            None => {
                eprintln!("Value not found... line:{} ", line);
                return None;
            }
        }
    }

    Some(answer)
}

fn get_largest_subsequence(s: &str, k: i32) -> Option<i64> {
    let n = s.len();
    let mut stack: Vec<i64> = Vec::with_capacity(k as usize);
    let digits: Vec<i64> = s
        .chars()
        .map(|x| match x.to_digit(10) {
            Some(value) => value as i64,
            None => 0,
        })
        .collect();

    for (i, digit) in digits.iter().enumerate() {
        while !stack.is_empty() && stack.len() + n - i > k as usize {
            let last: &i64 = match stack.last() {
                Some(last) => last,
                None => return None,
            };
            if last < digit {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k as usize {
            stack.push(*digit);
        }
    }
    let mut answer: i64 = 0;
    let ten: i64 = 10;

    for (i, digit) in stack.iter().rev().enumerate() {
        answer += digit * ten.pow(i as u32);
    }

    Some(answer)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day3";

    #[test]
    fn test_day3_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 357;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day3_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 17179;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day3_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 3121910778619;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day3_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 170025781683941;
        assert_eq!(expected, actual);
    }
}
