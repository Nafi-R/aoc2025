use crate::days::CalculateSolution;
use std::env;
use std::process;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Not enough arguments: {:?} <day1 or day2...>", args[0]);
        process::exit(1);
    }

    let day_name: &str = &args[1];
    let mut day = days::Day::default();
    match day_name {
        "day1" => {
            day.set_file("day1");
            day.set_day(days::DayNumber::Day1);
        }
        "day2" => {
            day.set_file("day2");
            day.set_day(days::DayNumber::Day2);
        }
        "day3" => {
            day.set_file("day3");
            day.set_day(days::DayNumber::Day3);
        }
        "day4" => {
            day.set_file("day4");
            day.set_day(days::DayNumber::Day4);
        }
        "day5" => {
            day.set_file("day5");
            day.set_day(days::DayNumber::Day5);
        }
        "day6" => {
            day.set_file("day6");
            day.set_day(days::DayNumber::Day6);
        }
        "day7" => {
            day.set_file("day7");
            day.set_day(days::DayNumber::Day7);
        }
        "day8" => {
            day.set_file("day8");
            day.set_day(days::DayNumber::Day8);
        }
        "day9" => {
            day.set_file("day9");
            day.set_day(days::DayNumber::Day9);
        }
        "day10" => {
            day.set_file("day10");
            day.set_day(days::DayNumber::Day10);
        }
        "day11" => {
            day.set_file("day11");
            day.set_day(days::DayNumber::Day11);
        }
        "day12" => {
            day.set_file("day12");
            day.set_day(days::DayNumber::Day12);
        }
        _ => {
            eprintln!("{:?} has not been implemented", day_name);
            return;
        }
    }
    day.part1();
    day.part2();
}
