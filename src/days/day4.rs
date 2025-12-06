static DIRECTIONS: [(i64, i64); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let graph: Vec<Vec<char>> = file_lines.map(|s| s.chars().collect()).collect();

    let answer: i64 = remove_rolls(graph, false);

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let graph: Vec<Vec<char>> = file_lines.map(|s| s.chars().collect()).collect();

    let answer: i64 = remove_rolls(graph, true);
    Some(answer)
}

fn count_roll(graph: &Vec<Vec<char>>, row: i64, col: i64) -> i64 {
    if row < 0 {
        return 0;
    }
    if col < 0 {
        return 0;
    }
    let row = row as usize;
    let col = col as usize;

    match graph.get(row) {
        Some(line) => match &line.get(col) {
            Some(character) => match &character {
                '@' => 1,
                _ => 0,
            },
            None => 0,
        },
        None => 0,
    }
}

fn remove_rolls(mut graph: Vec<Vec<char>>, is_recursive: bool) -> i64 {
    let mut n_removed = 0;
    let rows: usize = graph.len();
    let cols: usize = if let Some(cols) = graph.get(0) {
        cols.len()
    } else {
        eprintln!("Error: No columns found");
        return 0;
    };

    for row in 0..rows {
        for col in 0..cols {
            let mut adjacent_rolls = 0;

            if count_roll(&graph, row as i64, col as i64) == 0 {
                continue;
            }
            for (x_offset, y_offset) in DIRECTIONS {
                adjacent_rolls += count_roll(&graph, row as i64 + x_offset, col as i64 + y_offset);
            }
            if adjacent_rolls < 4 {
                n_removed += 1;
                if is_recursive {
                    graph[row][col] = '.';
                }
            }
        }
    }

    match n_removed {
        0 => 0,
        _ => match is_recursive {
            true => n_removed + remove_rolls(graph, true),
            false => n_removed,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day4";

    #[test]
    fn test_day4_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 13;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day4_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 1489;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day4_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 43;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day4_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 8890;
        assert_eq!(expected, actual);
    }
}
