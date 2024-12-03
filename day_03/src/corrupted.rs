use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn run(file_path: &str) {
    match read_data(file_path) {
        Ok(total) => {
            println!("total: {:?}", total);
        }
        Err(e) => {
            eprintln!("Error reading data: {}", e);
        }
    }
}

fn read_data(file_path: &str) -> Result<i32, io::Error> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let mut content = String::new();

    // Read the entire file into the string
    file.read_to_string(&mut content)?;

    // Process the full content
    let total = get_total(&content);
    Ok(total)
}

fn get_total(line: &str) -> i32 {
    let mut total = 0;
    // partial pattern
    let partial_pattern = Regex::new(r"^m(u(l(\(\d*(,\d*)?)?)?)?$").unwrap();
    // full pattern
    let full_pattern = Regex::new(r"mul\(\d*,\d*\)$").unwrap();
    let partial_enable = Regex::new(r"d(?:o(?:n(?:'?(?:t(?:\()?)?)?)?)?").unwrap();
    let full_enable = Regex::new(r"^(do|don't)\(\)$").unwrap();
    let mut expression = String::new();
    let mut enabled_expression = String::new();
    let mut enabled = true;
    let mut dos = 0;
    let mut donts = 0;
    for ch in line.chars() {
        let new_expression = format!("{}{}", expression, ch);
        let new_enabled_expression = format!("{}{}", enabled_expression, ch);
        if partial_enable.is_match(&new_enabled_expression) {
            enabled_expression = new_enabled_expression.clone();
            if let Some(captures) = full_enable.captures(&enabled_expression) {
                let keyword = captures.get(1).unwrap().as_str();
                if keyword == "do" {
                    if !enabled {
                        dos += 1;
                    }
                    enabled = true;
                } else if keyword == "don't" {
                    if enabled {
                        donts += 1;
                    }
                    enabled = false;
                }

                enabled_expression = String::new(); // Reset after handling
            }
        } else {
            enabled_expression = String::new();
        }
        if enabled {
            if partial_pattern.is_match(&new_expression) {
                expression = new_expression.clone();
            } else {
                expression = String::new();
            }
            if full_pattern.is_match(&new_expression) {
                let parts: Vec<&str> = new_expression
                    .split(|c| c == '(' || c == ')' || c == ',')
                    .filter(|s| !s.is_empty()) // Remove empty strings
                    .collect();
                println!("parts: {:?}", parts);
                if parts.len() == 3 {
                    if let (Ok(a), Ok(b)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                        total += a * b;
                    } else {
                        eprintln!("Invalid number: {}", new_expression);
                    }
                } else {
                    eprintln!("Invalid expression: {}", new_expression);
                }
                expression = String::new();
            }
        }
    }
    println!("dos: {}, donts: {}", dos, donts);
    total
}
