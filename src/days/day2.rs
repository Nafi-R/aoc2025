use aoc_utils::get_day_lines;

pub fn part1() {
    let file_lines: Vec<String> = get_day_lines("day2");
    let mut answer: i128 = 0;
    let line = if let Some(contents) = file_lines.get(0) {
        contents
    } else {
        eprintln!("No line found in day2.txt");
        return;
    };

    let entries: Vec<&str> = line.split(",").collect();

    for (i, entry) in entries.iter().enumerate() {
        let parts: Vec<&str> = entry.split("-").collect();

        let (start, end): (i64, i64) = if let [left_str, right_str] = parts.as_slice() {
            let left = left_str.parse::<i64>();
            let right = right_str.parse::<i64>();
            match (left, right) {
                (Ok(left), Ok(right)) => (left, right),
                (Err(_), Ok(_)) | (Ok(_), Err(_)) | (Err(_), Err(_)) => {
                    eprintln!(
                        "Error parsing line {}: '{}'. At least one value not a number",
                        i, entry
                    );
                    return;
                }
            }
        } else {
            eprintln!("Error parsing line {} : '{}'. No '-' given.", i, entry);
            return;
        };

        for num in start..(end + 1) {
            let num_str: String = format!("{}", num);
            let len = num_str.len();
            let mid_point = num_str
                .char_indices()
                .map(|(i, _)| i)
                .nth(num_str.chars().count() / 2)
                .unwrap_or(len);

            if &num_str[..mid_point] == &num_str[mid_point..] {
                answer += i128::from(num);
            }
        }
    }

    println!("Day2 Part 1 answer: {:?}", answer);
}

pub fn part2() {
    // let file_lines: Vec<String> = get_day_lines("day2");
    // let mut answer: i128 = 0;
    // println!("Day2 Part 2 answer: {:?}", count);
}
