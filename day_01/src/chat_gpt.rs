use std::collections::HashMap;

pub fn run(input: &str) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let line = line;
        let parts: Vec<_> = line.split_whitespace().collect();

        if let [a, b] = parts.as_slice() {
            match (a.parse::<i32>(), b.parse::<i32>()) {
                (Ok(a), Ok(b)) => {
                    list_a.push(a);
                    list_b.push(b);
                }
                _ => eprintln!("Invalid numbers in line: {}", line),
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    list_a.sort_unstable();
    list_b.sort_unstable();
    let similarity = get_similarity_score(&list_a, &list_b);
    println!("Similarity: {}", similarity);
}

// fn get_distance(list_a: &[i32], list_b: &[i32]) -> i32 {
//     list_a.iter().zip(list_b).map(|(a, b)| (a - b).abs()).sum()
// }

fn get_similarity_score(list_a: &[i32], list_b: &[i32]) -> i32 {
    let hash_b: HashMap<i32, i32> = list_b.iter().fold(HashMap::new(), |mut acc, &b| {
        *acc.entry(b).or_insert(0) += 1;
        acc
    });

    list_a
        .iter()
        .map(|&a| a * hash_b.get(&a).copied().unwrap_or(0))
        .sum()
}
