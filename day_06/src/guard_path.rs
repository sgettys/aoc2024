use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

// Define a hashmap to store the directions:
/*
^ -> (0, -1)
> -> (1, 0)
v -> (0, 1)
< -> (-1, 0)
*/
lazy_static! {  // Use lazy_static! macro
    static ref DIRECTIONS: HashMap<char, (i32, i32)> = {
        let mut m = HashMap::new();
        m.insert('^', (-1, 0));
        m.insert('>', (0, 1));
        m.insert('v', (1, 0));
        m.insert('<', (0, -1));
        m
    };
}

lazy_static! {
    static ref UP: char = '^';
    static ref RIGHT: char = '>';
    static ref DOWN: char = 'v';
    static ref LEFT: char = '<';
}

pub fn run(input: &str) {
    let starting_position = find_starting_position(input);
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dir = grid[starting_position.0 as usize][starting_position.1 as usize];

    // Save original grid state
    let original_grid = grid.clone();

    // First run to get steps
    grid[starting_position.0 as usize][starting_position.1 as usize] = 'X';
    let mut steps = 1;
    steps += walk_path(&mut grid, starting_position, &dir);
    println!("Steps: {}", steps);

    // Save the path grid to know where we can put obstructions
    let path_grid = grid.clone();

    let mut loops = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pos = (i as i32, j as i32);
            if path_grid[i][j] == 'X' && pos != starting_position {
                // Reset grid to original state for each test
                grid = original_grid.clone();
                grid[starting_position.0 as usize][starting_position.1 as usize] = 'X';

                // Put obstruction and test
                put_obstruction(&mut grid, (i as i32, j as i32));
                let loop_steps = walk_path(&mut grid, starting_position, &dir);
                if loop_steps == -1 {
                    loops += 1;
                }
            }
        }
    }
    println!("Loops: {}", loops);
}

fn find_starting_position(input: &str) -> (i32, i32) {
    for (i, row) in input.lines().enumerate() {
        for (j, c) in row.chars().enumerate() {
            // Check if the character exists as a key in DIRECTIONS
            if DIRECTIONS.contains_key(&c) {
                return (i as i32, j as i32);
            }
        }
    }
    (0, 0)
}

fn walk_path(grid: &mut Vec<Vec<char>>, starting_position: (i32, i32), direction: &char) -> i32 {
    let mut visited_states: HashSet<((i32, i32), char)> = HashSet::new();
    let mut i = starting_position.0;
    let mut j = starting_position.1;
    let mut current_direction = *direction;
    let mut steps = 0;
    while i < grid.len() as i32 && j < grid[i as usize].len() as i32 {
        let current_state = ((i, j), current_direction);
        if visited_states.contains(&current_state) {
            return -1;
        }
        visited_states.insert(current_state);
        let dir_coord = get_direction(&current_direction);
        let next_pos = (i + dir_coord.0, j + dir_coord.1);
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= grid.len() as i32
            || next_pos.1 >= grid[i as usize].len() as i32
        {
            return steps;
        }
        let next_char = grid[next_pos.0 as usize][next_pos.1 as usize];
        if next_char == '#' {
            current_direction = *turn_direction(&current_direction);
        } else {
            // Check if we've been here before
            if grid[next_pos.0 as usize][next_pos.1 as usize] == 'X' {
            } else {
                grid[next_pos.0 as usize][next_pos.1 as usize] = 'X';
                steps += 1;
            }
            i = next_pos.0;
            j = next_pos.1;
        }
    }
    steps
}

fn put_obstruction(grid: &mut Vec<Vec<char>>, position: (i32, i32)) {
    grid[position.0 as usize][position.1 as usize] = '#';
}
fn get_direction(input: &char) -> (i32, i32) {
    *DIRECTIONS.get(input).unwrap()
}
fn turn_direction(current: &char) -> &'static char {
    match *current {
        // Dereference current to match on the value
        '^' => &RIGHT,
        '>' => &DOWN,
        'v' => &LEFT,
        '<' => &UP,
        _ => &RIGHT, // Default case should also return a static reference
    }
}
