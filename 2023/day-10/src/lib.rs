pub mod part1 {
    // Define a Position struct to represent grid coordinates (i, j).
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    struct Position(usize, usize);

    // Enum to represent the four possible directions of movement in the grid.
    #[derive(Copy, Clone, Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    // Parses the input string into a 2D vector (grid) of characters. Each character represents
    // a part of the pipe or empty space. 'S' marks the starting position of the animal.
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    // Finds the starting position 'S' in the grid, returning its coordinates.
    fn find_start(grid: &[Vec<char>]) -> Option<Position> {
        for (i, row) in grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|&cell| cell == 'S') {
                return Some(Position(i, j));
            }
        }
        None
    }

    // Navigates the pipe loop starting from the position 'S' and returns the longest distance
    // from the start along the loop. This function handles the movement logic in the grid.
    fn navigate_loop(grid: &[Vec<char>], start: Position) -> Option<i32> {
        let mut position = start;
        let mut direction = initial_direction(grid, start)?;
        let mut steps = 0;

        loop {
            let current_tile = grid[position.0][position.1];

            // Handle movement based on the type of tile encountered.
            match current_tile {
                'S' | '-' | '|' => (), // Continue if it's a straight path or start.
                'L' | 'J' | '7' | 'F' => direction = bend_direction(grid, position, direction)?, // Change direction at bends.
                _ => return None, // Return None for invalid tiles.
            }

            // If returning to 'S' after moving, the longest distance is found.
            if steps > 0 && current_tile == 'S' {
                return Some(steps);
            }

            // Apply the current direction to move to the next position.
            position = apply_direction(direction, position);
            steps += 1;
        }
    }

    // Applies the current direction to the given position to get the next position.
    fn apply_direction(direction: Direction, position: Position) -> Position {
        match direction {
            Direction::Up => Position(position.0.wrapping_sub(1), position.1),
            Direction::Down => Position(position.0 + 1, position.1),
            Direction::Left => Position(position.0, position.1.wrapping_sub(1)),
            Direction::Right => Position(position.0, position.1 + 1),
        }
    }

    // Determines the initial direction of movement from the start position based on the
    // surrounding pipe configuration.
    fn initial_direction(grid: &[Vec<char>], start: Position) -> Option<Direction> {
        let (i, j) = (start.0, start.1);

        if i > 0 && connects_to(grid[i - 1][j], Direction::Up) {
            return Some(Direction::Up);
        }
        if i < grid.len() - 1 && connects_to(grid[i + 1][j], Direction::Down) {
            return Some(Direction::Down);
        }
        if j > 0 && connects_to(grid[i][j - 1], Direction::Left) {
            return Some(Direction::Left);
        }
        if j < grid[0].len() - 1 && connects_to(grid[i][j + 1], Direction::Right) {
            return Some(Direction::Right);
        }

        None
    }

    // Checks if a pipe type can connect in the given direction.
    fn connects_to(pipe: char, direction: Direction) -> bool {
        matches!(
            (pipe, direction),
            ('|', Direction::Up)
                | ('|', Direction::Down)
                | ('-', Direction::Left)
                | ('-', Direction::Right)
                | ('L', Direction::Down)
                | ('L', Direction::Left)
                | ('J', Direction::Down)
                | ('J', Direction::Right)
                | ('7', Direction::Up)
                | ('7', Direction::Right)
                | ('F', Direction::Up)
                | ('F', Direction::Left)
        )
    }

    // Determines the new direction after a bend based on the current direction
    // and the type of bend encountered.
    fn bend_direction(
        grid: &[Vec<char>],
        position: Position,
        direction: Direction,
    ) -> Option<Direction> {
        let (i, j) = (position.0, position.1);
        match grid[i][j] {
            'L' => match direction {
                Direction::Left => Some(Direction::Up),
                Direction::Down => Some(Direction::Right),
                _ => None,
            },
            'J' => match direction {
                Direction::Right => Some(Direction::Up),
                Direction::Down => Some(Direction::Left),
                _ => None,
            },
            '7' => match direction {
                Direction::Right => Some(Direction::Down),
                Direction::Up => Some(Direction::Left),
                _ => None,
            },
            'F' => match direction {
                Direction::Left => Some(Direction::Down),
                Direction::Up => Some(Direction::Right),
                _ => None,
            },
            _ => None,
        }
    }

    // Public function to find the longest distance in the pipe loop from the starting position.
    // It parses the input, finds the start position, and calculates the distance.
    pub fn find_longest_distance(input: &str) -> i32 {
        let grid = parse_input(input);
        let start = find_start(&grid).unwrap(); // Assuming there is always a valid start.
        (navigate_loop(&grid, start).unwrap_or_default() + 1) / 2
    }
}

pub mod part2 {
    // Define a Position struct to represent grid coordinates (i, j).
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    struct Position(usize, usize);

    // Enum to represent the four possible directions of movement in the grid.
    #[derive(Copy, Clone, Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    // Parses the input string into a 2D vector (grid) of characters. Each character represents
    // a part of the pipe or empty space. 'S' marks the starting position of the animal.
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    // Finds the starting position 'S' in the grid, returning its coordinates.
    fn find_start(grid: &[Vec<char>]) -> Option<Position> {
        for (i, row) in grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|&cell| cell == 'S') {
                return Some(Position(i, j));
            }
        }
        None
    }

    // Navigates the loop starting from the 'S' position.
    // It tracks the path taken and returns a vector of Positions representing this path.
    fn navigate_loop(grid: &[Vec<char>], start: Position) -> Option<Vec<Position>> {
        let mut position = start;
        let mut direction = initial_direction(grid, start)?;
        let mut steps = 0;
        let mut path = vec![start];

        loop {
            let current_tile = grid[position.0][position.1];

            path.push(position);

            // Handle movement based on the type of tile encountered.
            match current_tile {
                'S' | '-' | '|' => (), // Continue if it's a straight path or start.
                'L' | 'J' | '7' | 'F' => direction = bend_direction(grid, position, direction)?, // Change direction at bends.
                _ => return None, // Return None for invalid tiles.
            }

            // If returning to 'S' after moving, the longest distance is found.
            if steps > 0 && current_tile == 'S' {
                break;
            }

            // Apply the current direction to move to the next position.
            position = apply_direction(direction, position);
            steps += 1;
        }
        Some(path)
    }

    // Applies the current direction to the given position to get the next position.
    fn apply_direction(direction: Direction, position: Position) -> Position {
        match direction {
            Direction::Up => Position(position.0.wrapping_sub(1), position.1),
            Direction::Down => Position(position.0 + 1, position.1),
            Direction::Left => Position(position.0, position.1.wrapping_sub(1)),
            Direction::Right => Position(position.0, position.1 + 1),
        }
    }

    // Determines the initial direction of movement from the start position based on the
    // surrounding pipe configuration.
    fn initial_direction(grid: &[Vec<char>], start: Position) -> Option<Direction> {
        let (i, j) = (start.0, start.1);

        if i > 0 && connects_to(grid[i - 1][j], Direction::Up) {
            return Some(Direction::Up);
        }
        if i < grid.len() - 1 && connects_to(grid[i + 1][j], Direction::Down) {
            return Some(Direction::Down);
        }
        if j > 0 && connects_to(grid[i][j - 1], Direction::Left) {
            return Some(Direction::Left);
        }
        if j < grid[0].len() - 1 && connects_to(grid[i][j + 1], Direction::Right) {
            return Some(Direction::Right);
        }

        None
    }

    // Checks if a pipe type can connect in the given direction.
    fn connects_to(pipe: char, direction: Direction) -> bool {
        matches!(
            (pipe, direction),
            ('|', Direction::Up)
                | ('|', Direction::Down)
                | ('-', Direction::Left)
                | ('-', Direction::Right)
                | ('L', Direction::Down)
                | ('L', Direction::Left)
                | ('J', Direction::Down)
                | ('J', Direction::Right)
                | ('7', Direction::Up)
                | ('7', Direction::Right)
                | ('F', Direction::Up)
                | ('F', Direction::Left)
        )
    }

    // Determines the new direction after a bend based on the current direction
    // and the type of bend encountered.
    fn bend_direction(
        grid: &[Vec<char>],
        position: Position,
        direction: Direction,
    ) -> Option<Direction> {
        let (i, j) = (position.0, position.1);
        match grid[i][j] {
            'L' => match direction {
                Direction::Left => Some(Direction::Up),
                Direction::Down => Some(Direction::Right),
                _ => None,
            },
            'J' => match direction {
                Direction::Right => Some(Direction::Up),
                Direction::Down => Some(Direction::Left),
                _ => None,
            },
            '7' => match direction {
                Direction::Right => Some(Direction::Down),
                Direction::Up => Some(Direction::Left),
                _ => None,
            },
            'F' => match direction {
                Direction::Left => Some(Direction::Down),
                Direction::Up => Some(Direction::Right),
                _ => None,
            },
            _ => None,
        }
    }

    // Calculate the area enclosed by a loop using the Surveyor's (Shoelace) formula.
    // This formula is especially useful for irregular polygons and works by:
    // - Iterating over the vertices of the polygon (loop path in this case).
    // - For each pair of vertices, compute the cross product of their coordinates.
    // - Sum these products and divide by 2 to get the absolute area.
    // The vertices are provided as a list of (x, y) coordinate tuples.
    fn surveyors_formula(vertices: &[(usize, usize)]) -> f64 {
        let mut area: f64 = 0.0;
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            // Cross product of coordinates: (x[i] * y[j]) - (y[i] * x[j])
            let area_contribution =
                (vertices[i].0 * vertices[j].1) as f64 - (vertices[j].0 * vertices[i].1) as f64;
            area += area_contribution;
        }
        // The absolute value of half the cross product sum gives the area
        area.abs() / 2.0
    }

// Public function to calculate the number of tiles enclosed by the loop.
// It uses the Surveyor's formula to calculate the area of the loop and then applies Pick's theorem.
// Pick's theorem relates the area of a grid-based polygon to its interior and boundary lattice points:
// A = I + 0.5 * B - 1, where A is the area, I is the number of interior points, and B is the number of boundary points.
// Since we are interested only in the internal vertices (I), we rearrange Pick's theorem:
// I = A - 0.5 * B + 1
// Here, we count the boundary points as the vertices in the loop. However, we slightly modify the formula:
// As the loop may pass through some boundary points without enclosing them, we subtract 0.5 * vertices.len() instead.
// This modification ensures an accurate count of internal tiles, excluding those on the perimeter.
pub fn num_enclosed_tiles(input: &str) -> u32 {
    let grid = parse_input(input);
    let start = find_start(&grid).unwrap(); // Find the starting point of the loop
    let loop_vertices = navigate_loop(&grid, start).unwrap_or(vec![]); // Traverse the loop to get vertices

    // Convert loop vertices (Positions) to tuples of coordinates for area calculation
    let vertices = loop_vertices
        .iter()
        .map(|&Position(i, j)| (i, j))
        .collect::<Vec<_>>();

    // Calculate area enclosed by the loop using the Surveyor's formula
    let area = surveyors_formula(&vertices);

    // Applying the modified Pick's theorem to calculate the number of internal lattice points (tiles)
    let internal_vtx_count = area + 1.5 - 0.5 * vertices.len() as f64;
    internal_vtx_count.round() as u32 // Round the result to get the count of tiles
}

}
