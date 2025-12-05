use std::fs::File;
use std::io;
use std::io::BufRead;

static PATH_PREFIX: &str = "data/";

fn get_file_path(day_name: &str) -> String {
    format!("./{}{}.txt", PATH_PREFIX, day_name)
}

pub fn get_day_lines(day_name: &str) -> Option<impl Iterator<Item = String>> {
    let file_path = get_file_path(day_name);

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return None;
        }
    };
    let reader = io::BufReader::new(file);

    let line_iterator = reader.lines().filter_map(|line_result| match line_result {
        Ok(line) => Some(line),
        Err(e) => {
            eprintln!("Warning: Skipping line due to I/O error: {:?}", e);
            None
        }
    });

    Some(line_iterator)
}
