use aoc_utils::read_file;
use std::env;
use std::process;

static PATH_PREFIX: &str = "data/";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Not enough arguments: {:?} <day1 or day2...>", args[0]);
        process::exit(1);
    }

    let file_name = &args[1];

    let file_path = format!("./{}{}.txt", PATH_PREFIX, file_name);

    match read_file(&file_path) {
        Ok(lines) => {
            for line in lines {
                println!("{:?}", line);
            }
        }
        Err(_) => {
            eprintln!("Could not find file: {:?}", file_path);
        }
    }
}
