use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: &str) {
    let dimensions = get_dimensions(&input);
    println!("Dimensions: {:?}", dimensions);
    let frequencies = get_frequencies(&input);
    let mut antinodes = HashSet::new();
    for (frequency, positions) in frequencies {
        println!("{}: {:?}", frequency, positions);
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (antinode_a, antinode_b) = find_antinodes(positions[i], positions[j]);
                if is_valid_antinodes(dimensions, antinode_a) {
                    println!("Valid antinode: {:?}", antinode_a);
                    antinodes.insert(antinode_a);
                } else {
                    println!("Invalid antinode: {:?}", antinode_a);
                }
                if is_valid_antinodes(dimensions, antinode_b) {
                    println!("Valid antinode: {:?}", antinode_b);
                    antinodes.insert(antinode_b);
                } else {
                    println!("Invalid antinode: {:?}", antinode_b);
                }
            }
        }
    }
    println!("Antinodes: {:?}", antinodes.len());
    println!("{:?}", antinodes);
}
fn get_dimensions(input: &str) -> (i32, i32) {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    (width, height)
}

fn get_frequencies(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '.' {
                frequencies
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    frequencies
}

fn is_valid_antinodes(grid_size: (i32, i32), antinode_coordinates: (i32, i32)) -> bool {
    antinode_coordinates.0 >= 0
        && antinode_coordinates.0 < grid_size.0
        && antinode_coordinates.1 >= 0
        && antinode_coordinates.1 < grid_size.1
}

fn find_node_distance(first: (i32, i32), second: (i32, i32)) -> i32 {
    ((second.0 - first.0).abs() + (second.1 - first.1).abs())
}
fn find_antinodes(frequency_a: (i32, i32), frequency_b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = frequency_b.0 - frequency_a.0;
    let dy = frequency_b.1 - frequency_a.1;

    let antinode_a = (frequency_b.0 + dx, frequency_b.1 + dy);
    let antinode_b = (frequency_a.0 - dx, frequency_a.1 - dy);

    (antinode_a, antinode_b)
}
