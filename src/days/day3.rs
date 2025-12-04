use aoc_utils::get_day_lines;

pub fn part1() {
    let file_lines = match get_day_lines("day3") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };
    let mut answer: u64 = 0;
    for line in file_lines {
        match get_largest_subsequence(&line, 2) {
            Some(value) => {
                answer += value;
            }
            None => println!("Value not found... line:{} ", line),
        }
    }

    println!("Day3 Part 1 answer: {:?}", answer);
}

pub fn part2() {
    let file_lines = match get_day_lines("day3") {
        Some(file_lines) => file_lines,
        None => {
            return;
        }
    };
    let mut answer: u64 = 0;

    for line in file_lines {
        match get_largest_subsequence(&line, 12) {
            Some(value) => {
                answer += value;
            }
            None => println!("Value not found... line:{} ", line),
        }
    }

    println!("Day3 Part 2 answer: {:?}", answer);
}

fn get_largest_subsequence(s: &str, k: u32) -> Option<u64> {
    let n = s.len();
    let mut stack: Vec<u64> = Vec::with_capacity(k as usize);
    let digits: Vec<u64> = s
        .chars()
        .map(|x| match x.to_digit(10) {
            Some(value) => value as u64,
            None => 0,
        })
        .collect();

    for (i, digit) in digits.iter().enumerate() {
        while !stack.is_empty() && stack.len() + n - i > k as usize {
            let last: &u64 = match stack.last() {
                Some(last) => last,
                None => return None,
            };
            if last < digit {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k as usize {
            stack.push(*digit);
        }
    }
    let mut answer: u64 = 0;
    let ten: u64 = 10;

    for (i, digit) in stack.iter().rev().enumerate() {
        answer += digit * ten.pow(i as u32);
    }

    Some(answer)
}
