use utils::get_day_lines;

pub fn part1() -> Option<i64> {
    let mut file_lines = match get_day_lines("day2") {
        Some(file_lines) => file_lines,
        None => {
            return None;
        }
    };
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

pub fn part2() {
    let mut file_lines = match get_day_lines("day2") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };
    let mut answer: i128 = 0;
    let line = if let Some(contents) = file_lines.next() {
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
                _ => {
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
            for i in 0..len / 2 {
                if len % (i + 1) == 0 {
                    let reps = len / (i + 1);
                    if &num_str[..i + 1].repeat(reps) == &num_str {
                        answer += i128::from(num);
                        break;
                    }
                }
            }
        }
    }

    println!("Day2 Part 2 answer: {:?}", answer);
}
