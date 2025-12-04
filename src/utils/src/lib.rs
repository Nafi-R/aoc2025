use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

static PATH_PREFIX: &str = "data/";

fn get_file_path(day_name: &str) -> String {
    format!("./{}{}.txt", PATH_PREFIX, day_name)
}

fn read_file(file_path: &str) -> Option<Lines<BufReader<File>>> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening file: '{}'", file_path);
            return None;
        }
    };
    let reader = BufReader::new(file);
    Some(reader.lines())
}

pub fn get_day_lines(day_name: &str) -> Option<impl Iterator<Item = String>> {
    let file_name = get_file_path(day_name);

    match read_file(&file_name) {
        Some(line_iterator) => {
            let lines = line_iterator.filter_map(|line_result| match line_result {
                Ok(line) => Some(line), // Success: Map to Some(line)
                Err(e) => {
                    eprintln!("Cannot parse line: {:?}", e);
                    None
                }
            });

            // Return the complex, but lazy, FilterMap iterator.
            Some(lines)
        }
        None => None,
    }
}
