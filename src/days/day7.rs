use std::collections::{HashMap, LinkedList};

pub fn part1(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut answer: i64 = 0;
    let file_lines: Vec<String> = file_lines.collect();
    let start_line: &String = match file_lines.get(0) {
        Some(line) => line,
        None => {
            eprintln!("Error: No first line found");
            return None;
        }
    };

    let start_pos = match start_line.find("S") {
        Some(pos) => pos,
        None => {
            eprintln!("Error: No 'S' found on first line");
            return None;
        }
    };

    let n: usize = file_lines.len();

    let mut beams: LinkedList<usize> = LinkedList::from([start_pos]);

    for row in 1..n {
        let n_beams: usize = beams.len();

        for _ in 0..n_beams {
            let curr_beam: usize = beams.pop_back().unwrap();

            let space = match file_lines.get(row)?.chars().nth(curr_beam) {
                Some(space) => space,
                None => {
                    eprintln!(
                        "Error: Out of bounds found on row {} with beam at pos: '{}'",
                        row, curr_beam
                    );
                    return None;
                }
            };

            match space {
                '^' => {
                    if !beams.contains(&(curr_beam + 1)) {
                        beams.push_front(curr_beam + 1);
                    }
                    if !beams.contains(&(curr_beam - 1)) {
                        beams.push_front(curr_beam - 1);
                    }
                    answer += 1;
                }
                '.' => {
                    if !beams.contains(&(curr_beam)) {
                        beams.push_front(curr_beam);
                    }
                }
                _ => {
                    eprintln!("Unknown character found");
                    return None;
                }
            }
        }
    }

    Some(answer)
}

pub fn part2(file_lines: impl Iterator<Item = String>) -> Option<i64> {
    let mut answer: i64 = 0;
    let file_lines: Vec<String> = file_lines.collect();
    let start_line: &String = match file_lines.get(0) {
        Some(line) => line,
        None => {
            eprintln!("Error: No first line found");
            return None;
        }
    };

    let start_pos = match start_line.find("S") {
        Some(pos) => pos,
        None => {
            eprintln!("Error: No 'S' found on first line");
            return None;
        }
    };

    let n: usize = file_lines.len();

    let mut beams: LinkedList<usize> = LinkedList::from([start_pos]);
    let mut beam_counts: HashMap<usize, i64> = HashMap::from([(start_pos, 1)]);

    for row in 1..n {
        let n_beams: usize = beams.len();

        for _ in 0..n_beams {
            let curr_beam: usize = beams.pop_back().unwrap();

            let space = match file_lines.get(row)?.chars().nth(curr_beam) {
                Some(space) => space,
                None => {
                    eprintln!(
                        "Error: Out of bounds found on row {} with beam at pos: '{}'",
                        row, curr_beam
                    );
                    return None;
                }
            };

            match space {
                '^' => {
                    if !beams.contains(&(curr_beam + 1)) {
                        beams.push_front(curr_beam + 1);
                    }
                    if !beams.contains(&(curr_beam - 1)) {
                        beams.push_front(curr_beam - 1);
                    }

                    let beams_arriving = match beam_counts.remove_entry(&curr_beam) {
                        Some(entry) => entry.1,
                        None => {
                            eprintln!("Error: Somehow there's no beams entering here");
                            return None;
                        }
                    };

                    // Update for curr_beam + 1
                    *beam_counts.entry(curr_beam + 1).or_insert(0) += beams_arriving;

                    // Update for curr_beam - 1
                    *beam_counts.entry(curr_beam - 1).or_insert(0) += beams_arriving;
                }
                '.' => {
                    beams.push_front(curr_beam);
                }
                _ => {
                    eprintln!("Unknown character found");
                    return None;
                }
            }
        }
    }

    for beam in beams {
        match beam_counts.get(&beam) {
            Some(n_realities) => {
                answer += n_realities;
            }
            None => (),
        }
    }

    Some(answer)
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::get_day_lines;
    static DAY_STR: &str = "day7";

    #[test]
    fn test_day7_part1_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 21;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day7_part1_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part1(file_lines).unwrap();
        let expected = 1516;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day7_part2_example_input() {
        let file_path = format!("{}{}", DAY_STR, "_test");
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 40;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day7_part2_full_input() {
        let file_path = format!("{}", DAY_STR);
        let file_lines = get_day_lines(&file_path).unwrap();
        let actual = part2(file_lines).unwrap();
        let expected = 1393669447690;
        assert_eq!(expected, actual);
    }
}
