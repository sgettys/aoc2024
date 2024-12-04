pub fn run(input: &str) {
    let grid = parse_grid(input);
    let word = "XMAS";
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down right
        (-1, -1), // up left
        (1, -1),  // down left
        (-1, 1),  // up right
    ];
    let mut found = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != word.chars().next().unwrap() {
                continue;
            }
            for direction in directions.iter() {
                if search(&grid, word, (row as i32, col as i32), *direction) {
                    found += 1;
                }
            }
        }
    }
    println!("Found {} occurrences of {}", found, word);

    // part 2
    let word_2 = "MAS";
    let directions_2 = [
        (1, 1),  // down right
        (-1, 1), // up right
    ];
    let mut found_xmas = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != word_2.chars().next().unwrap()
                && grid[row][col] != word_2.chars().last().unwrap()
            {
                continue;
            }
            // Looking for the X of MAS so start at the same begining but only search down and right and if found then go down 2 rows and search up and right
            if (search(&grid, word_2, (row as i32, col as i32), directions_2[0])
                || search(
                    &grid,
                    word_2.chars().rev().collect::<String>().as_str(),
                    (row as i32, col as i32),
                    directions_2[0],
                ))
                && (search(&grid, word_2, (row as i32 + 2, col as i32), directions_2[1])
                    || search(
                        &grid,
                        word_2.chars().rev().collect::<String>().as_str(),
                        (row as i32 + 2, col as i32),
                        directions_2[1],
                    ))
            {
                found_xmas += 1;
            }
        }
    }
    println!("Found {} X occurrences of {}", found_xmas, word_2);
}

// This is a backtracking problem. We need to find a word in a 2D grid of characters.
// We can move in any direction and can visit the same cell more than once. Words can be overlapping
// and can start from any cell. We need to find all occurrences of the word in the grid.

// This function needs to parse the input string into a 2d grid of characters
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Recursively search for the word in the grid. If the next char in the word is found then continue searching in that direction otherwise
// return to the starting point and search in another direction.
fn search(grid: &Vec<Vec<char>>, word: &str, start: (i32, i32), direction: (i32, i32)) -> bool {
    let (mut row, mut col) = start;
    let (drow, dcol) = direction;
    let mut word = word.chars();
    let mut next = word.next();
    while let Some(next_char) = next {
        if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
            return false;
        }
        if grid[row as usize][col as usize] != next_char {
            return false;
        }
        row += drow;
        col += dcol;
        next = word.next();
    }
    true
}
