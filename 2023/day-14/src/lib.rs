pub mod part1 {
    use rayon::prelude::*;

    const BATCH_SIZE: usize = 100;

    pub fn tilt_and_sum_load(input: &str) -> usize {
        let columns = transpose_input(input);
        calculate_total_load(columns, input.lines().next().unwrap().len())
    }

    fn transpose_input(input: &str) -> Vec<char> {
        let lines: Vec<&str> = input.lines().collect();
        let num_rows = lines.len();
        let num_columns = lines[0].len();

        let mut transposed = vec!['.'; num_rows * num_columns];

        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                transposed[col * num_rows + row] = char;
            }
        }

        transposed
    }

    fn calculate_total_load(columns: Vec<char>, num_rows: usize) -> usize {
        // Create ranges for each batch and debug print them
        let batch_ranges: Vec<_> = (0..columns.len() / num_rows)
            .step_by(BATCH_SIZE)
            .map(|start_col| {
                start_col * num_rows..((start_col + BATCH_SIZE) * num_rows).min(columns.len())
            })
            .collect();

        // Process each batch in parallel
        batch_ranges
            .into_par_iter()
            .map(|range| {
                (range.start..range.end)
                    .step_by(num_rows)
                    .map(|col_start| {
                        let mut total_load = 0;
                        let mut next_load = num_rows;

                        for row in 0..num_rows {
                            match columns[col_start + row] {
                                'O' => {
                                    total_load += next_load;
                                    next_load -= 1;
                                }
                                '#' => next_load = num_rows - row - 1,
                                _ => (),
                            }
                        }

                        total_load
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

pub mod part2 {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};
    
    pub fn tilt_and_sum_load(input: &str) -> usize {
        let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut seen_states = HashSet::new();
        let total_cycles = 1_000_000_000;
        let mut cycle_start = 0;
    
        for cycle in 1.. {
            flip_up(&mut grid);
            flip_left(&mut grid);
            flip_down(&mut grid);
            flip_right(&mut grid);
    
            let state_hash = hash_grid(&grid);
            if !seen_states.insert(state_hash) {
                cycle_start = cycle;
                break;
            }
        }
    
        // Directly calculate cycle length
        let cycle_length = calculate_cycle_length(&mut grid);
    
        let effective_cycle = (total_cycles - cycle_start) % cycle_length + cycle_start;
        for _ in cycle_start..effective_cycle {
            flip_up(&mut grid);
            flip_left(&mut grid);
            flip_down(&mut grid);
            flip_right(&mut grid);
        }
    
        calculate_north_support_load(&grid)
    }

    fn flip_up(grid: &mut Vec<Vec<char>>) {
        for x in 0..grid[0].len() {
            let mut binding = 0;
            for y in 0..grid.len() {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    grid[binding][x] = 'O';
                    binding += 1;
                } else if grid[y][x] == '#' {
                    binding = y + 1;
                }
            }
        }
    }

    fn flip_left(grid: &mut Vec<Vec<char>>) {
        for y in 0..grid.len() {
            let mut binding = 0;
            for x in 0..grid[0].len() {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    grid[y][binding] = 'O';
                    binding += 1;
                } else if grid[y][x] == '#' {
                    binding = x + 1;
                }
            }
        }
    }

    fn flip_down(grid: &mut Vec<Vec<char>>) {
        for x in 0..grid[0].len() {
            let mut binding = (grid.len() - 1) as i32;
            for y in (0..grid.len()).rev() {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    grid[binding as usize][x] = 'O';
                    binding -= 1;
                } else if grid[y][x] == '#' {
                    binding = y as i32 - 1;
                }
            }
        }
    }

    fn flip_right(grid: &mut Vec<Vec<char>>) {
        for y in 0..grid.len() {
            let mut binding = grid[0].len() as i32 - 1;
            for x in (0..grid[0].len()).rev() {
                if grid[y][x] == 'O' {
                    grid[y][x] = '.';
                    grid[y][binding as usize] = 'O';
                    binding -= 1;
                } else if grid[y][x] == '#' {
                    binding = x as i32 - 1;
                }
            }
        }
    }

    fn calculate_north_support_load(grid: &[Vec<char>]) -> usize {
        let mut load = 0;
        let grid_height = grid.len();
    
        // Iterate over each column.
        for x in 0..grid[0].len() {
            for y in (0..grid_height).rev() {
                if grid[y][x] == 'O' {
                    // Load is the distance from the 'O' to the bottom of the grid.
                    load += grid_height - y;
                }
            }
        }
        load
    }
    

    fn hash_grid(grid: &[Vec<char>]) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        grid.hash(&mut hasher);
        hasher.finish()
    }

    fn calculate_cycle_length(grid: &mut Vec<Vec<char>>) -> usize {
        let initial_state_hash = hash_grid(grid);
        let mut cycle_count = 0;
    
        loop {
            cycle_count += 1;
            flip_up(grid);
            flip_left(grid);
            flip_down(grid);
            flip_right(grid);
    
            let current_state_hash = hash_grid(grid);
            if current_state_hash == initial_state_hash {
                break;
            }
        }
    
        cycle_count
    }
}
