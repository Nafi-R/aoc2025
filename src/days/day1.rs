pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut pos = 50;
    let mut zero_counter = 0;
    for line in file_lines {
        let is_positive: bool = &line[..1] == "R";
        if let Ok(rotation) = &line[1..].parse::<i64>() {
            if is_positive {
                pos = (pos + rotation) % 100;
            } else {
                pos = (pos - rotation) % 100;
            }
            if pos == 0 {
                zero_counter += 1;
            }
        }
    }
    Some(zero_counter)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut curr_pos: i64 = 50;
    let mut zero_counter: i64 = 0;
    for line in file_lines {
        let is_positive: bool = &line[..1] == "R";
        if let Ok(rotation) = &line[1..].parse::<i64>() {
            if is_positive {
                zero_counter += (curr_pos + rotation) / 100;
                curr_pos = (curr_pos + rotation) % 100;
            } else {
                // Modulo in Rust results in a negative. Needs to be added to 100 to wrap correctly
                let mut new_pos = (curr_pos - rotation) % 100;
                if new_pos < 0 {
                    new_pos = new_pos + 100;
                }

                if curr_pos == 0 {
                    zero_counter += rotation / 100;
                } else if rotation > &curr_pos {
                    zero_counter += ((rotation - curr_pos - 1) / 100) + 1;
                    if new_pos == 0 {
                        zero_counter += 1;
                    }
                } else if rotation == &curr_pos {
                    zero_counter += 1;
                }
                curr_pos = new_pos;
            }
        }
    }
    Some(zero_counter)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day1";

    #[test]
    fn test_day1_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 3;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day1_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 1043;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day1_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 6;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day1_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 5963;
        assert_eq!(expected, actual);
    }
}
