use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(file_path: &str) {
    match read_data(file_path) {
        Ok(safe) => {
            println!("safe: {:?}", safe);
        }
        Err(e) => {
            eprintln!("Error reading data: {}", e);
        }
    }
}

fn read_data(file_path: &str) -> Result<i32, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe = 0;
    for line in reader.lines() {
        let mut numbers = Vec::new();
        let line = line?;
        for num in line.split_whitespace() {
            if let Ok(number) = num.parse::<i32>() {
                numbers.push(number);
            } else {
                eprintln!("Invalid number: {}", num);
            }
        }
        if is_data_safe(&numbers) {
            safe += 1;
        } else {
        }
    }
    Ok(safe)
}

fn is_data_safe(data: &[i32]) -> bool {
    if data.len() < 2 || data[0] == data[1] {
        return false;
    }
    let incr = data[1] > data[0];
    for window in data.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 {
            return false; // Data must be changing
        }
        if diff.abs() > 3 {
            return false; // at least 1 but no more than 3
        }
        if incr && diff < 0 || !incr && diff > 0 {
            return false; // direction must not change
        }
    }
    true
}
