pub mod part1 {
    pub fn count_xmas_occurrences(input: &str) -> u32 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let word = ['X', 'M', 'A', 'S'];
        let mut count = 0;

        // Directions: N, NE, E, SE, S, SW, W, NW
        let directions = [
            (-1, 0),  // North
            (-1, 1),  // North-East
            (0, 1),   // East
            (1, 1),   // South-East
            (1, 0),   // South
            (1, -1),  // South-West
            (0, -1),  // West
            (-1, -1), // North-West
        ];

        for x in 0..n {
            for y in 0..m {
                for &(dx, dy) in &directions {
                    if search_from(&grid, x as isize, y as isize, dx, dy, &word) {
                        count += 1;
                    }
                }
            }
        }

        count as u32
    }

    fn search_from(
        grid: &[Vec<char>],
        mut x: isize,
        mut y: isize,
        dx: isize,
        dy: isize,
        word: &[char],
    ) -> bool {
        let n = grid.len() as isize;
        let m = grid[0].len() as isize;

        for &ch in word {
            if x < 0 || x >= n || y < 0 || y >= m {
                return false;
            }
            if grid[x as usize][y as usize] != ch {
                return false;
            }
            x += dx;
            y += dy;
        }
        true
    }
}

pub mod part2 {
    pub fn count_xmas_occurrences(input: &str) -> u32 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let n = grid.len();
        if n < 3 {
            return 0;
        }
        let m = grid[0].len();
        if m < 3 {
            return 0;
        }

        let mut count = 0;

        // Define the MAS sequences (start and end characters only)
        let mas_sequences = [
            (('M', 'S'), ('M', 'S')), // Both diagonals forwards
            (('S', 'M'), ('S', 'M')), // Both diagonals backwards
            (('M', 'S'), ('S', 'M')), // One forward, one backward
            (('S', 'M'), ('M', 'S')), // One backward, one forward
        ];

        for x in 1..n - 1 {
            for y in 1..m - 1 {
                if grid[x][y] != 'A' {
                    continue;
                }

                for &((d1_start_char, d1_end_char), (d2_start_char, d2_end_char)) in &mas_sequences
                {
                    if check_diagonal(&grid, x, y, -1, -1, d1_start_char, d1_end_char)
                        && check_diagonal(&grid, x, y, -1, 1, d2_start_char, d2_end_char)
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn check_diagonal(
        grid: &[Vec<char>],
        x0: usize,
        y0: usize,
        dx: isize,
        dy: isize,
        start_char: char,
        end_char: char,
    ) -> bool {
        let n = grid.len() as isize;
        let m = grid[0].len() as isize;

        let x = x0 as isize;
        let y = y0 as isize;

        let x1 = x + dx;
        let y1 = y + dy;
        let x3 = x - dx;
        let y3 = y - dy;

        if x1 < 0 || x1 >= n || y1 < 0 || y1 >= m || x3 < 0 || x3 >= n || y3 < 0 || y3 >= m {
            return false;
        }

        grid[x1 as usize][y1 as usize] == start_char && grid[x3 as usize][y3 as usize] == end_char
    }
}
