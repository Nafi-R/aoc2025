use std::fs::File;
use std::io::{self, BufRead, BufReader};

static PATH_PREFIX: &str = "data/";

fn get_file_path(day_name: &str) -> String {
    format!("./{}{}.txt", PATH_PREFIX, day_name)
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn get_day_lines(day_name: &str) -> Vec<String> {
    let file_name = get_file_path(day_name);
    match read_file(&file_name) {
        Ok(lines) => {
            println!("Reading file: {}", file_name);
            lines
        }
        Err(e) => {
            eprintln!("Day date file cannot be found. {:?}", e);
            Vec::new()
        }
    }
}
