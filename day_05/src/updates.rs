use std::collections::HashMap;

pub fn run(input: &str) {
    let (order, updates) = split_input(input);
    let map = get_update_order(&order);
    let mut total = 0;
    let mut sorted_total = 0;
    for page in updates.split_whitespace().collect::<Vec<&str>>() {
        let result = is_in_order(page, &map);
        if result {
            // Get the middle number of the page.
            let nums = page
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let middle = nums[nums.len() / 2];
            total += middle;
        } else {
            let result = set_in_order(page, &map);
            let middle = result[result.len() / 2];
            sorted_total += middle;
        }
    }
    println!("total: {}", total);
    println!("sorted total: {}", sorted_total);
}

fn split_input(input: &str) -> (String, String) {
    let mut order = String::new();
    let mut updates = String::new();
    for line in input.lines() {
        if line.contains("|") {
            order.push_str(line);
            order.push('\n');
        } else {
            updates.push_str(line);
            updates.push('\n');
        }
    }
    (order, updates)
}

// I'm really feeling like this is a trie problem. I think I need to build out tries for each update order and then merge them all into a single trie at the end.
// The update input is in this order:
/*
47|53
97|13
97|61
75|29
61|13
etc...
*/
// I think that starting with the top number I can build out the trie for each dependent update. I"ll keep a main trie that has all of the updates merged into it and if the current update isn't in the main trie I will build out a new trie for it.
// or just use a hashmap to store the updates and then build out the order from there.
fn get_update_order(input: &str) -> HashMap<i32, Vec<i32>> {
    let mut map = HashMap::new();
    for line in input.split_whitespace().collect::<Vec<&str>>() {
        let parts = line.split("|").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let key = parts[0].parse::<i32>().unwrap();
            map.entry(key).or_insert(Vec::new());
            let value = parts[1].parse::<i32>().unwrap();
            map.entry(key).or_insert(Vec::new()).push(value);
        }
    }
    map
}

fn is_in_order(page: &str, order: &HashMap<i32, Vec<i32>>) -> bool {
    let nums = page
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 0..nums.len() {
        // Check if any of the numbers in the order hash appear before the current number.
        let current: i32 = nums[i];
        if let Some(must_be_before) = order.get(&current) {
            for n in must_be_before {
                if nums[..i].contains(&n) {
                    return false;
                }
            }
        } else {
            //println!("no order for {}", current);
        }
    }
    true
}

fn set_in_order(page: &str, order: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut nums = page
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut i = 0;
    while i < nums.len() {
        let current: i32 = nums[i];
        if let Some(must_be_before) = order.get(&current) {
            let mut found = false;
            for n in must_be_before {
                if nums[..i].contains(&n) {
                    // Move current back 1 place and test again
                    let temp = nums[i - 1];
                    nums[i - 1] = current;
                    nums[i] = temp;
                    i = 0;
                    found = true;
                    break;
                }
            }
            if !found {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    nums
}
