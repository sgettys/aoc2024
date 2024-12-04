use regex::Regex;

// YUCK
pub fn parse_and_compute(line: &str) {
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
    for ch in line.chars() {
        let new_expression = format!("{}{}", expression, ch);
        let new_enabled_expression = format!("{}{}", enabled_expression, ch);
        if partial_enable.is_match(&new_enabled_expression) {
            enabled_expression = new_enabled_expression.clone();
            if let Some(captures) = full_enable.captures(&enabled_expression) {
                let keyword = captures.get(1).unwrap().as_str();
                if keyword == "do" {
                    if !enabled {}
                    enabled = true;
                } else if keyword == "don't" {
                    if enabled {}
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
    println!("Total: {}", total);
}
