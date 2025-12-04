use utils::get_day_lines;

static DIRECTIONS: [(i32, i32); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

pub fn part1() {
    let file_lines = match get_day_lines("day4") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

    let graph: Vec<Vec<char>> = file_lines.map(|s| s.chars().collect()).collect();

    let answer: i32 = remove_rolls(graph, false);

    println!("Day4 Part 1 answer: {:?}", answer);
}

pub fn part2() {
    let file_lines = match get_day_lines("day4") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

    let graph: Vec<Vec<char>> = file_lines.map(|s| s.chars().collect()).collect();

    let answer: i32 = remove_rolls(graph, true);

    println!("Day4 Part 2 answer: {:?}", answer);
}

fn count_roll(graph: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {
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

fn remove_rolls(mut graph: Vec<Vec<char>>, is_recursive: bool) -> i32 {
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

            if count_roll(&graph, row as i32, col as i32) == 0 {
                continue;
            }
            for (x_offset, y_offset) in DIRECTIONS {
                adjacent_rolls += count_roll(&graph, row as i32 + x_offset, col as i32 + y_offset);
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
