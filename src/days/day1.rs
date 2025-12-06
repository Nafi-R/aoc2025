use utils::get_day_lines;

pub fn part1() -> Option<i64> {
    let file_lines = match get_day_lines("day1") {
        Some(file_lines) => file_lines,
        None => {
            return None;
        }
    };
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

pub fn part2() -> Option<i64> {
    let file_lines = match get_day_lines("day1") {
        Some(file_lines) => file_lines,
        None => {
            return None;
        }
    };
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
    use super::part1;
    use super::part2;

    #[test]
    fn test_day1_part1() {
        let expected = 1043;
        let result = part1().expect("Day 1 Part 1 could not read file");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_day1_part2() {
        let expected = 5963;
        let result = part2().expect("Day 1 Part 2 could not read file");
        assert_eq!(expected, result);
    }
}
