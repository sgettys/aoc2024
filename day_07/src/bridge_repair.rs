pub fn run(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let (test_value, numbers) = parse_line(line);
        let sum = numbers[0] + numbers[1];
        let product = numbers[0] * numbers[1];
        let concat = get_concat(numbers[0], numbers[1]);
        if is_solvable(test_value, sum, &numbers[2..])
            || is_solvable(test_value, product, &numbers[2..])
            || is_solvable(test_value, concat, &numbers[2..])
        {
            total += test_value;
            println!("{}", line);
        }
    }
    println!("{}", total);
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let parts = line.split(":").collect::<Vec<&str>>();
    let test_value = parts[0].parse::<i64>().unwrap();
    let numbers = parts[1]
        .trim()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    (test_value, numbers)
}

fn is_solvable(target: i64, current: i64, numbers: &[i64]) -> bool {
    // Base cases
    if numbers.len() == 0 {
        return current == target;
    }

    // Try addition
    let sum = current + numbers[0];
    let concat = get_concat(current, numbers[0]);
    // Try multiplication with overflow check
    let product = match current.checked_mul(numbers[0]) {
        Some(p) => p,
        None => {
            // If multiplication would overflow, just try addition and concat
            return sum <= target && is_solvable(target, sum, &numbers[1..])
                || concat <= target && is_solvable(target, concat, &numbers[1..]);
        }
    };

    // Try addition, multiplication, and concat
    (sum <= target && is_solvable(target, sum, &numbers[1..]))
        || (product <= target && is_solvable(target, product, &numbers[1..]))
        || (concat <= target && is_solvable(target, concat, &numbers[1..]))
}

fn get_concat(a: i64, b: i64) -> i64 {
    let mut s = a.to_string();
    s.push_str(&b.to_string());
    s.parse::<i64>().unwrap()
}
