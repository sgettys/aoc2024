use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: &str) {
    let dimensions = get_dimensions(&input);
    println!("Dimensions: {:?}", dimensions);
    let frequencies = get_frequencies(&input);
    let mut antinodes = HashSet::new();
    let mut all_antinodes = HashSet::new();

    for (frequency, positions) in frequencies {
        println!("{}: {:?}", frequency, positions);
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                // Find initial antinodes
                let (antinode_a, antinode_b) = find_antinodes(positions[i], positions[j]);

                // Add valid antinodes to set
                if is_valid_antinodes(dimensions, antinode_a) {
                    println!("Valid antinode: {:?}", antinode_a);
                    antinodes.insert(antinode_a);
                }
                if is_valid_antinodes(dimensions, antinode_b) {
                    println!("Valid antinode: {:?}", antinode_b);
                    antinodes.insert(antinode_b);
                }

                // Find inline antinodes from original positions
            }
        }
        if positions.len() >= 2 {
            // Only process frequencies with 2 or more antennas
            let collinear = find_collinear_points(dimensions, &positions);
            all_antinodes.extend(collinear);
        }
    }

    // Remove any invalid inline antinodes

    println!("Antinodes: {}", antinodes.len());

    println!("Linear antinodes: {}", all_antinodes.len());
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

fn find_antinodes(frequency_a: (i32, i32), frequency_b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = frequency_b.0 - frequency_a.0;
    let dy = frequency_b.1 - frequency_a.1;

    let antinode_a = (frequency_b.0 + dx, frequency_b.1 + dy);
    let antinode_b = (frequency_a.0 - dx, frequency_a.1 - dy);

    (antinode_a, antinode_b)
}
// Find inline antinodes
fn is_collinear(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    // Check if three points are in a straight line
    // Using the formula: (y2 - y1)(x3 - x1) = (y3 - y1)(x2 - x1)
    (p2.1 - p1.1) * (p3.0 - p1.0) == (p3.1 - p1.1) * (p2.0 - p1.0)
}

fn find_collinear_points(grid_size: (i32, i32), points: &[(i32, i32)]) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    // For each pair of points
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            // Add the points themselves as they're part of a collinear set
            if is_valid_antinodes(grid_size, points[i]) {
                antinodes.insert(points[i]);
            }
            if is_valid_antinodes(grid_size, points[j]) {
                antinodes.insert(points[j]);
            }

            // Check all grid points between and beyond these points
            let dx = points[j].0 - points[i].0;
            let dy = points[j].1 - points[i].1;

            // Extend the line in both directions
            for t in -grid_size.0..=grid_size.0 {
                let x = points[i].0 + (dx * t);
                let y = points[i].1 + (dy * t);
                let point: (i32, i32) = (x, y);

                if is_valid_antinodes(grid_size, point) {
                    antinodes.insert(point);
                }
            }
        }
    }

    antinodes
}
