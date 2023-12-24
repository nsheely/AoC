pub mod part1 {
    // Define constants for directions using bit shifts for efficient representation.
    pub const NORTH: u8 = 1;
    pub const EAST: u8 = 2;
    pub const SOUTH: u8 = 4;
    pub const WEST: u8 = 8;

    // Entry point to calculate the number of tiles energized by the light beam.
    pub fn num_energized(input: &str) -> usize {
        // Create a grid from the input string.
        let grid = parse_input(input);
        // Count the number of energized tiles.
        energize_count( &grid, 0, 0, EAST)
    }

    // Convert the input string into a 2D grid of characters.
    pub fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines()
             .map(|line| line.trim().chars().collect())
             .filter(|v: &Vec<char>| !v.is_empty())
             .collect()
    }

    // Function to count the number of tiles energized by the light beam.
    pub fn energize_count(grid: &Vec<Vec<char>>, start_x: i64, start_y: i64, start_dir: u8) -> usize {
        let (width, height) = (grid[0].len(), grid.len());
        let mut total = 0;
        // Use a 2D vector to track the state of each tile. Each byte can represent all 4 directions.
        let mut light = vec![vec![0u8; width]; height];
        // Use a vector as a stack to hold the positions and directions to process.
        let mut queue = Vec::new(); 
    
        // Starting position is the top-left corner, moving EAST.
        queue.push((start_y, start_x, start_dir));
    
        while let Some((y, x, dir_bit)) = queue.pop() {
            if x < 0 || y < 0 || x >= width as i64 || y >= height as i64 {
                continue;
            }
            let (yi, xi) = (y as usize, x as usize);
    
            // Skip if this tile in this direction has already been processed.
            if light[yi][xi] & dir_bit != 0 {
                continue;
            }
    
            // Mark this direction as processed for this tile.
            light[yi][xi] |= dir_bit;
            // Increment total only if this is the first time any beam has hit this tile.
            if light[yi][xi] == dir_bit {
                total += 1;
            }
    
            // Determine the next positions and directions based on the current tile.
            let cell = grid[yi][xi];
            match dir_bit {
                NORTH => match cell {
                    '.' | '|' => queue.push((y - 1, x, NORTH)),
                    '/' => queue.push((y, x + 1, EAST)),
                    '\\' => queue.push((y, x - 1, WEST)),
                    '-' => {
                        queue.push((y, x + 1, EAST));
                        queue.push((y, x - 1, WEST));
                    },
                    _ => (),
                },
                SOUTH => match cell {
                    '.' | '|' => queue.push((y + 1, x, SOUTH)),
                    '/' => queue.push((y, x - 1, WEST)),
                    '\\' => queue.push((y, x + 1, EAST)),
                    '-' => {
                        queue.push((y, x + 1, EAST));
                        queue.push((y, x - 1, WEST));
                    },
                    _ => (),
                },
                EAST => match cell {
                    '.' | '-' => queue.push((y, x + 1, EAST)),
                    '/' => queue.push((y - 1, x, NORTH)),
                    '\\' => queue.push((y + 1, x, SOUTH)),
                    '|' => {
                        queue.push((y - 1, x, NORTH));
                        queue.push((y + 1, x, SOUTH));
                    },
                    _ => (),
                },
                WEST => match cell {
                    '.' | '-' => queue.push((y, x - 1, WEST)),
                    '/' => queue.push((y + 1, x, SOUTH)),
                    '\\' => queue.push((y - 1, x, NORTH)),
                    '|' => {
                        queue.push((y - 1, x, NORTH));
                        queue.push((y + 1, x, SOUTH));
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    
        total
    }
}

pub mod part2 {
    use super::part1::{energize_count, parse_input, NORTH, EAST, SOUTH, WEST};
    use rayon::prelude::*;

    // Finds the maximum number of tiles that can be energized by the light beam using parallel computation.
    pub fn max_energized(input: &str) -> usize {
        let grid = parse_input(input);
        let (width, height) = (grid[0].len() as i64, grid.len() as i64);

        // Create a vector of all edge positions
        let mut edge_positions = Vec::new();
        for x in 0..width {
            edge_positions.push((x, 0, SOUTH)); // Top row, heading downward
            edge_positions.push((x, height - 1, NORTH)); // Bottom row, heading upward
        }
        for y in 0..height {
            edge_positions.push((0, y, EAST)); // Left column, heading right
            edge_positions.push((width - 1, y, WEST)); // Right column, heading left
        }

        // Use rayon to process the edge positions in parallel
        edge_positions.par_iter()
            .map(|&(x, y, dir)| energize_count(&grid, x, y, dir))
            .max()
            .unwrap_or(0)
    }
}
