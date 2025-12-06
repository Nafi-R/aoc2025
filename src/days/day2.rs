pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut file_lines = file_lines;
    let mut answer: i64 = 0;
    let line = if let Some(contents) = file_lines.next() {
        contents
    } else {
        eprintln!("No line found in day2.txt");
        return None;
    };

    let entries: Vec<&str> = line.split(",").collect();

    for (i, entry) in entries.iter().enumerate() {
        let parts: Vec<&str> = entry.split("-").collect();

        let (start, end): (i64, i64) = if let [left_str, right_str] = parts.as_slice() {
            let left = left_str.parse::<i64>();
            let right = right_str.parse::<i64>();
            match (left, right) {
                (Ok(left), Ok(right)) => (left, right),
                _ => {
                    eprintln!(
                        "Error parsing line {}: '{}'. At least one value not a number",
                        i, entry
                    );
                    return None;
                }
            }
        } else {
            eprintln!("Error parsing line {} : '{}'. No '-' given.", i, entry);
            return None;
        };

        for num in start..(end + 1) {
            let num_str: String = format!("{}", num);
            let mid_point = num_str.len() / 2;
            if &num_str[..mid_point] == &num_str[mid_point..] {
                answer += num;
            }
        }
    }

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut file_lines = file_lines;
    let mut answer: i64 = 0;
    let line = if let Some(contents) = file_lines.next() {
        contents
    } else {
        eprintln!("No line found in day2.txt");
        return None;
    };

    let entries: Vec<&str> = line.split(",").collect();

    for (i, entry) in entries.iter().enumerate() {
        let parts: Vec<&str> = entry.split("-").collect();

        let (start, end): (i64, i64) = if let [left_str, right_str] = parts.as_slice() {
            let left = left_str.parse::<i64>();
            let right = right_str.parse::<i64>();
            match (left, right) {
                (Ok(left), Ok(right)) => (left, right),
                _ => {
                    eprintln!(
                        "Error parsing line {}: '{}'. At least one value not a number",
                        i, entry
                    );
                    return None;
                }
            }
        } else {
            eprintln!("Error parsing line {} : '{}'. No '-' given.", i, entry);
            return None;
        };

        for num in start..(end + 1) {
            let num_str: String = format!("{}", num);
            let len = num_str.len();
            for i in 0..len / 2 {
                if len % (i + 1) == 0 {
                    let reps = len / (i + 1);
                    if &num_str[..i + 1].repeat(reps) == &num_str {
                        answer += num;
                        break;
                    }
                }
            }
        }
    }

    Some(answer)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day2";

    #[test]
    fn test_day2_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 1227775554;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day2_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 15873079081;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day2_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 4174379265;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day2_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 22617871034;
        assert_eq!(expected, actual);
    }
}
