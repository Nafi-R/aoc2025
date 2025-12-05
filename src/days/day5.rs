use utils::get_day_lines;

pub fn part1() {
    let file_lines = match get_day_lines("day5") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

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
                    return;
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
                    return;
                }
            }
        }
    }

    let mut answer: i32 = 0;

    for ingredient_id in ingredient_ids {
        for (start, end) in &ranges {
            if ingredient_id >= *start && ingredient_id <= *end {
                answer += 1;
                break;
            }
        }
    }

    println!("Day5 Part 1 answer: {:?}", answer);
}

pub fn part2() {
    let file_lines = match get_day_lines("day5") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };

    let mut ranges: Vec<(i128, i128)> = Vec::new();

    for (i, line) in file_lines.enumerate() {
        if line.contains("-") {
            let parts: Vec<&str> = line.split("-").collect();
            if parts.len() < 2 {
                continue;
            }
            match (parts[0].parse::<i128>(), parts[1].parse::<i128>()) {
                (Ok(start), Ok(end)) => {
                    if end < start {
                        eprintln!(
                            "Error parsing line {}, end must be greater or equal to start: '{}'",
                            i, line
                        );
                        return;
                    }
                    ranges.push((start, end));
                }
                _ => {
                    eprintln!("Error parsing line {}, expecting range: '{}'", i, line);
                    return;
                }
            };
        }
    }

    let mut answer: i128 = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut last_end: i128 = 0;
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
                panic!("Something went wrong here for range: {}=>{}", start, end);
            }
        }
    }

    println!("Day5 Part 2 answer: {:?}", answer);
}
