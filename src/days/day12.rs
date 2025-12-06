pub fn part1(_file_lines: impl Iterator<Item = String>) -> Option<i64> {
    todo!();
}

pub fn part2(_file_lines: impl Iterator<Item = String>) -> Option<i64> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day12";

    #[ignore]
    #[test]
    fn test_day12_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 4277556;
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_day12_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 7644505810277;
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_day12_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 3263827;
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_day12_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 12841228084455;
        assert_eq!(expected, actual);
    }
}
