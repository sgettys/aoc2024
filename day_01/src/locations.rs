use std::collections::HashMap;

pub fn run(input: &str) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for line in input.lines() {
        let line = line;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                list_a.push(a);
                list_b.push(b);
            } else {
                eprintln!("Invalid number: {}", line);
            }
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }
    list_a.sort();
    list_b.sort();
    //let distance = get_distance(&list_a, &list_b);
    let similarity = get_similarity_score(&list_a, &list_b);
    println!("Similarity: {}", similarity);
}

// fn get_distance(list_a: &[i32], list_b: &[i32]) -> i32 {
//     let mut distance = 0;
//     for (a, b) in list_a.iter().zip(list_b.iter()) {
//         distance += (a - b).abs();
//     }
//     distance
// }

fn get_similarity_score(list_a: &[i32], list_b: &[i32]) -> i32 {
    let mut hash_b = HashMap::new();
    let mut distance = 0;
    for b in list_b.iter() {
        *hash_b.entry(b).or_insert(0) += 1;
    }
    for a in list_a.iter() {
        let similarity = *hash_b.get(a).unwrap_or(&0);
        distance += a * similarity;
    }
    distance
}
