use std::time::Instant;
use utils::get_day_lines;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Debug, Default)]
pub enum DayNumber {
    #[default]
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

#[derive(Default, Debug)]
pub struct Day {
    file_name: String,
    day_num: DayNumber,
}

pub trait CalculateSolution {
    fn part1(&self);
    fn part2(&self);
}

impl Day {
    fn get_input(&self) -> Option<impl Iterator<Item = String>> {
        get_day_lines(&self.file_name)
    }

    pub fn set_file(&mut self, file_path: &str) {
        self.file_name = file_path.into();
    }

    pub fn set_day(&mut self, day_number: DayNumber) {
        self.day_num = day_number;
    }

    pub fn get_file(&self) -> &str {
        return &self.file_name;
    }
}

impl CalculateSolution for Day {
    fn part1(&self) {
        let input = match self.get_input() {
            Some(input) => input,
            None => panic!("Could not read file: {}", self.file_name),
        };
        let now = Instant::now();

        let result: Option<i64> = match self.day_num {
            DayNumber::Day1 => day1::part1(input),
            DayNumber::Day2 => day2::part1(input),
            DayNumber::Day3 => day3::part1(input),
            DayNumber::Day4 => day4::part1(input),
            DayNumber::Day5 => day5::part1(input),
            DayNumber::Day6 => day6::part1(input),
            DayNumber::Day7 => day7::part1(input),
            DayNumber::Day8 => day8::part1(input),
            DayNumber::Day9 => day9::part1(input),
            DayNumber::Day10 => day10::part1(input),
            DayNumber::Day11 => day11::part1(input),
            DayNumber::Day12 => day12::part1(input),
        };
        let elapsed = now.elapsed().as_millis();

        let answer = match result {
            Some(result) => result,
            None => {
                eprintln!("No output found for {:?}: ", self.day_num);
                return;
            }
        };

        println!("{:?} => Part 1 ({} ms): {}", self.day_num, elapsed, answer);
    }

    fn part2(&self) {
        let input = match self.get_input() {
            Some(input) => input,
            None => panic!("Could not read file: {}", self.file_name),
        };

        let now = Instant::now();
        let result: Option<i64> = match self.day_num {
            DayNumber::Day1 => day1::part2(input),
            DayNumber::Day2 => day2::part2(input),
            DayNumber::Day3 => day3::part2(input),
            DayNumber::Day4 => day4::part2(input),
            DayNumber::Day5 => day5::part2(input),
            DayNumber::Day6 => day6::part2(input),
            DayNumber::Day7 => day7::part2(input),
            DayNumber::Day8 => day8::part2(input),
            DayNumber::Day9 => day9::part2(input),
            DayNumber::Day10 => day10::part2(input),
            DayNumber::Day11 => day11::part2(input),
            DayNumber::Day12 => day12::part2(input),
        };
        let elapsed = now.elapsed().as_millis();
        let answer = match result {
            Some(result) => result,
            None => {
                eprintln!("No output found for {:?}: ", self.day_num);
                return;
            }
        };

        println!("{:?} => Part 2 ({} ms): {}", self.day_num, elapsed, answer);
    }
}
