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
                let (antinode_a, antinode_b) =
                    find_antinodes(dimensions, positions[i], positions[j]);
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
    // make sure the antinode is within the grid
    antinode_coordinates.0 >= 0
        && antinode_coordinates.0 < grid_size.0
        && antinode_coordinates.1 >= 0
        && antinode_coordinates.1 < grid_size.1
}

fn find_node_distance(grid_size: (i32, i32), first: (i32, i32), second: (i32, i32)) -> i32 {
    // Convert 2D coordinates to 1D index
    let first_index = (first.0 * grid_size.1) + first.1;
    let second_index = (second.0 * grid_size.1) + second.1;

    // Get absolute difference
    (second_index - first_index).abs()
}

fn find_antinodes(
    grid_size: (i32, i32),
    frequency_a: (i32, i32),
    frequency_b: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let distance = find_node_distance(grid_size, frequency_a, frequency_b);

    let positive = walk_to_coordinate(grid_size, frequency_b, distance);
    let negative = walk_to_coordinate(grid_size, frequency_a, -distance);
    (positive, negative)
}

fn walk_to_coordinate(grid_size: (i32, i32), start: (i32, i32), positions: i32) -> (i32, i32) {
    // Calculate total grid positions
    let total_pos = start.0 * grid_size.1 + start.1 + positions;

    // Calculate new row and column
    let row = total_pos / grid_size.1;
    let column = total_pos % grid_size.1;

    (row, column)
}
