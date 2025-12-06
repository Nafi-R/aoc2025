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
    match day_name {
        "day1" => {
            days::day1::part1();
            days::day1::part2();
        }
        "day2" => {
            days::day2::part1();
            days::day2::part2();
        }
        "day3" => {
            days::day3::part1();
            days::day3::part2();
        }
        "day4" => {
            days::day4::part1();
            days::day4::part2();
        }
        "day5" => {
            days::day5::part1();
            days::day5::part2();
        }
        "day6" => {
            days::day6::part1();
            days::day6::part2();
        }
        _ => eprintln!("{:?} has not been implemented", day_name),
    }
}
