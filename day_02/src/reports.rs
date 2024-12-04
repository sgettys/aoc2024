pub fn run(input: &str) {
    let mut safe = 0;
    for line in input.lines() {
        let mut numbers = Vec::new();
        let line = line;
        for num in line.split_whitespace() {
            if let Ok(number) = num.parse::<i32>() {
                numbers.push(number);
            } else {
                eprintln!("Invalid number: {}", num);
            }
        }
        if is_data_safe(&numbers) || dampen_data_safe(&numbers) {
            safe += 1;
        } else {
        }
    }
    println!("Safe: {}", safe);
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

// Don't like this brute force approach.. Need to use backtracking
fn dampen_data_safe(data: &[i32]) -> bool {
    for i in 0..data.len() {
        // slice out the current number
        let mut slice = data.to_vec();
        slice.remove(i);
        if is_data_safe(&slice) {
            return true;
        }
    }
    false
}
