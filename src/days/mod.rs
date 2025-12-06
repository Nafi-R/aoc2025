use std::fmt;
use utils::get_day_lines;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub enum DayNumber {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
}

impl fmt::Display for DayNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            DayNumber::Day1 => write!(f, "{}", "Day1"),
            DayNumber::Day2 => write!(f, "{}", "Day2"),
            DayNumber::Day3 => write!(f, "{}", "Day3"),
            DayNumber::Day4 => write!(f, "{}", "Day4"),
            DayNumber::Day5 => write!(f, "{}", "Day5"),
            DayNumber::Day6 => write!(f, "{}", "Day6"),
            DayNumber::Day7 => write!(f, "{}", "Day7"),
            DayNumber::Day8 => write!(f, "{}", "Day8"),
            DayNumber::Day9 => write!(f, "{}", "Day9"),
            DayNumber::Day10 => write!(f, "{}", "Day10"),
            DayNumber::Day11 => write!(f, "{}", "Day11"),
            DayNumber::Day12 => write!(f, "{}", "Day12"),
        }
    }
}

pub struct Day {
    file_name: String,
    day_num: DayNumber,
}

pub trait CalculateSolution {
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

impl Day {
    fn get_input(&self) -> Option<impl Iterator<Item = String>> {
        get_day_lines(&self.file_name)
    }
}

impl CalculateSolution for Day {
    fn part1(&self) -> i64 {
        let input = match self.get_input() {
            Some(input) => input,
            None => panic!("Could not read file: {}", self.file_name),
        };

        let result: Option<i64> = match self.day_num {
            DayNumber::Day1 => day1::part1(),
            DayNumber::Day2 => day2::part1(),
            DayNumber::Day3 => day3::part1(),
            DayNumber::Day4 => day4::part1(),
            DayNumber::Day5 => day5::part1(),
            _ => 0,
        };

        match result {
            Some(result) => result,
            None => {
                eprintln!("No output found for {}: ", self.day_num);
                return 0;
            }
        }
    }

    fn part2(&self) -> i64 {
        let input = match self.get_input() {
            Some(input) => input,
            None => panic!("Could not read file: {}", self.file_name),
        };
    }
}
