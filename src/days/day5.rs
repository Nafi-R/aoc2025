pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ingredient_ids: Vec<i64> = Vec::new();

    for (i, line) in file_lines.enumerate() {
        if line.contains("-") {
            let parts: Vec<&str> = line.split("-").collect();
            if parts.len() < 2 {
                continue;
            }
            match (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                (Ok(start), Ok(end)) => {
                    ranges.push((start, end));
                }
                _ => {
                    eprintln!("Error parsing line {} expecting range: '{}'", i, line);
                    return None;
                }
            };
        } else {
            if line.len() == 0 {
                continue;
            }

            match line.parse::<i64>() {
                Ok(value) => {
                    ingredient_ids.push(value);
                }
                _ => {
                    eprintln!(
                        "Error parsing line {} expecting ingredient ID: '{}'",
                        i, line
                    );
                    return None;
                }
            }
        }
    }

    let mut answer: i64 = 0;

    for ingredient_id in ingredient_ids {
        for (start, end) in &ranges {
            if ingredient_id >= *start && ingredient_id <= *end {
                answer += 1;
                break;
            }
        }
    }

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for (i, line) in file_lines.enumerate() {
        if line.contains("-") {
            let parts: Vec<&str> = line.split("-").collect();
            if parts.len() < 2 {
                continue;
            }
            match (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                (Ok(start), Ok(end)) => {
                    if end < start {
                        eprintln!(
                            "Error parsing line {}, end must be greater or equal to start: '{}'",
                            i, line
                        );
                        return None;
                    }
                    ranges.push((start, end));
                }
                _ => {
                    eprintln!("Error parsing line {}, expecting range: '{}'", i, line);
                    return None;
                }
            };
        }
    }

    let mut answer: i64 = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut last_end: i64 = 0;
    for (start, end) in &ranges {
        match last_end {
            n if n < *start => {
                answer += (end - start) + 1;
                last_end = *end;
            }
            n if n >= *start && n <= *end => {
                answer += end - last_end;
                last_end = *end;
            }
            n if n > *end => {
                continue;
            }
            _ => {
                eprintln!("Something went wrong here for range: {}=>{}", start, end);
                return None;
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day5";

    #[test]
    fn test_day5_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 3;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day5_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 664;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day5_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 14;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day5_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 350780324308385;
        assert_eq!(expected, actual);
    }
}
