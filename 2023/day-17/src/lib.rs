pub mod part1 {
    use std::collections::{BinaryHeap, HashSet};

    /// Represents the possible directions of movement.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        /// Applies this direction to the given point, returning the new point.
        /// Calculates the next point based on the current direction.
        pub fn apply(self, point: Point) -> Option<Point> {
            match self {
                Direction::Up => point.y.checked_sub(1).map(|y| Point { x: point.x, y }),
                Direction::Down => Some(Point { x: point.x, y: point.y + 1 }),
                Direction::Left => point.x.checked_sub(1).map(|x| Point { x, y: point.y }),
                Direction::Right => Some(Point { x: point.x + 1, y: point.y }),
            }
        }
    }

    /// Represents a state in the pathfinding algorithm.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    struct State {
        current: Point,
        direction: Direction,
        direction_count: usize,
    }

    /// Represents a point on the grid.
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    /// Represents a path including its current state, cost, and heuristic.
    /// 'cost' is the accumulated heat loss so far, and 'heuristic' is the estimated cost to reach the goal.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct Path {
        state: State,
        cost: usize,
        heuristic: usize,
    }

    impl Path {
        /// Generates the next possible paths from the current state, taking into account the movement constraints and avoiding backtracking.
        fn next_paths(&self, grid: &[Vec<usize>], goal: Point) -> Vec<Path> {
            let mut paths = Vec::with_capacity(3); // Capacity reduced to 3 as backtracking is not allowed
            let directions = match self.state.direction {
                // Choose directions that don't lead back to the previous block.
                Direction::Up => &[Direction::Up, Direction::Left, Direction::Right],
                Direction::Down => &[Direction::Down, Direction::Left, Direction::Right],
                Direction::Left => &[Direction::Left, Direction::Up, Direction::Down],
                Direction::Right => &[Direction::Right, Direction::Up, Direction::Down],
            };
    
            for &direction in directions {
                if let Some(next_point) = direction.apply(self.state.current) {
                    let new_direction_count = if direction == self.state.direction {
                        self.state.direction_count + 1
                    } else {
                        1
                    };
    
                    // Ensuring the crucible doesn't move more than three blocks in the same direction.
                    if new_direction_count <= 3 {
                        if let Some(cost) = get_value(grid, next_point) {
                            let total_cost = self.cost + cost;
                            let heuristic = manhattan_distance(next_point, goal);
                            paths.push(Path {
                                state: State {
                                    current: next_point,
                                    direction,
                                    direction_count: new_direction_count,
                                },
                                cost: total_cost,
                                heuristic,
                            });
                        }
                    }
                }
            }
    
            paths
        }
    }
    
    impl Ord for Path {
        /// Compares paths for ordering in the priority queue.
        /// Paths with lower total (cost + heuristic) are considered higher priority.
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
                .then_with(|| self.state.current.x.cmp(&other.state.current.x))
                .then_with(|| self.state.current.y.cmp(&other.state.current.y))
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    /// Parses the input string into a 2D grid of heat loss values.
    fn parse_input(input: &str) -> Vec<Vec<usize>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
            .collect()
    }

    /// Retrieves the heat loss value for a given point in the grid.
    fn get_value(grid: &[Vec<usize>], point: Point) -> Option<usize> {
        grid.get(point.y).and_then(|row| row.get(point.x).copied())
    }

    /// Calculates the Manhattan distance between two points.
    /// Used as a heuristic for estimating the remaining cost to the goal.
    fn manhattan_distance(p1: Point, p2: Point) -> usize {
        (p1.x as isize - p2.x as isize).unsigned_abs() + (p1.y as isize - p2.y as isize).unsigned_abs()
    }

    /// Solves the least heat loss problem using a modified A* algorithm.
    pub fn least_heat_loss(input: &str) -> usize {
        let grid = parse_input(input);
        let grid_height = grid.len();
        let grid_width = grid.first().map_or(0, Vec::len);
        let mut visited = HashSet::new(); // Tracks visited states to avoid revisiting.
    
        // Define the first starting state (moving to the right)
        let start_state_right = State {
            current: Point { x: 0, y: 0 },
            direction: Direction::Right,
            direction_count: 0,
        };
        visited.insert(start_state_right); // Insert the first starting state
    
        // Define the second starting state (moving down)
        let start_state_down = State {
            current: Point { x: 0, y: 0 },
            direction: Direction::Down,
            direction_count: 0,
        };
        visited.insert(start_state_down); // Insert the second starting state
    
        let mut heap = BinaryHeap::new(); // Priority queue for exploring paths.
    
        // Push both starting states into the heap
        heap.push(Path {
            state: start_state_right,
            cost: 0,
            heuristic: manhattan_distance(start_state_right.current, Point { x: grid_width - 1, y: grid_height - 1 }),
        });
        heap.push(Path {
            state: start_state_down,
            cost: 0,
            heuristic: manhattan_distance(start_state_down.current, Point { x: grid_width - 1, y: grid_height - 1 }),
        });
    
        while let Some(current_path) = heap.pop() {
            if current_path.state.current.x == grid_width - 1
                && current_path.state.current.y == grid_height - 1
            {
                // Termination condition: Reached the goal.
                return current_path.cost;
            }
    
            // Generate and explore next possible paths.
            current_path.next_paths(&grid, Point { x: grid_width - 1, y: grid_height - 1 })
                .into_iter()
                .filter(|path| visited.insert(path.state))
                .for_each(|path| heap.push(path));
        }
    
        usize::MAX // No valid path found.
    }
}

pub mod part2 {
    use std::collections::{BinaryHeap, HashSet};

    /// Represents the possible directions of movement.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        /// Applies this direction to the given point, returning the new point.
        /// Calculates the next point based on the current direction.
        pub fn apply(self, point: Point) -> Option<Point> {
            match self {
                Direction::Up => point.y.checked_sub(1).map(|y| Point { x: point.x, y }),
                Direction::Down => Some(Point { x: point.x, y: point.y + 1 }),
                Direction::Left => point.x.checked_sub(1).map(|x| Point { x, y: point.y }),
                Direction::Right => Some(Point { x: point.x + 1, y: point.y }),
            }
        }
    }

    /// Represents a state in the pathfinding algorithm.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    struct State {
        current: Point,
        direction: Direction,
        direction_count: usize,
    }

    /// Represents a point on the grid.
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    /// Represents a path including its current state, cost, and heuristic.
    /// 'cost' is the accumulated heat loss so far, and 'heuristic' is the estimated cost to reach the goal.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct Path {
        state: State,
        cost: usize,
        heuristic: usize,
    }

    impl Path {
        /// Generates the next possible paths from the current state, taking into account the movement constraints and avoiding backtracking.
        fn next_paths(&self, grid: &[Vec<usize>], goal: Point) -> Vec<Path> {
            let min_steps = 4;
            let max_steps = 10;
            let mut paths = Vec::with_capacity(3); // Capacity reduced to 3 as backtracking is not allowed
    
            if self.state.direction_count < min_steps {
                let directions = &[self.state.direction]; // Allow only the current direction
                for &direction in directions {
                    if let Some(next_point) = direction.apply(self.state.current) {
                        let new_direction_count = self.state.direction_count + 1;
    
                        if let Some(cost) = get_value(grid, next_point) {
                            let total_cost = self.cost + cost;
                            let heuristic = manhattan_distance(next_point, goal);
                            paths.push(Path {
                                state: State {
                                    current: next_point,
                                    direction,
                                    direction_count: new_direction_count,
                                },
                                cost: total_cost,
                                heuristic,
                            });
                        }
                    }
                }
            } else {
                let directions = match self.state.direction {
                    // Choose directions that don't lead back to the previous block.
                    Direction::Up => &[Direction::Up, Direction::Left, Direction::Right],
                    Direction::Down => &[Direction::Down, Direction::Left, Direction::Right],
                    Direction::Left => &[Direction::Left, Direction::Up, Direction::Down],
                    Direction::Right => &[Direction::Right, Direction::Up, Direction::Down],
                };
    
                for &direction in directions {
                    if let Some(next_point) = direction.apply(self.state.current) {
                        let new_direction_count = if direction == self.state.direction {
                            self.state.direction_count + 1
                        } else {
                            1
                        };
    
                        // Ensuring the crucible doesn't move more than three blocks in the same direction.
                        if new_direction_count <= max_steps {
                            if let Some(cost) = get_value(grid, next_point) {
                                let total_cost = self.cost + cost;
                                let heuristic = manhattan_distance(next_point, goal);
                                let new_direction = direction; // Create a separate variable for direction
                                paths.push(Path {
                                    state: State {
                                        current: next_point,
                                        direction: new_direction,
                                        direction_count: new_direction_count,
                                    },
                                    cost: total_cost,
                                    heuristic,
                                });
                            }
                        }
                    }
                }
            }
    
            paths
        }
    }
    
    
    impl Ord for Path {
        /// Compares paths for ordering in the priority queue.
        /// Paths with lower total (cost + heuristic) are considered higher priority.
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
                .then_with(|| self.state.current.x.cmp(&other.state.current.x))
                .then_with(|| self.state.current.y.cmp(&other.state.current.y))
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    /// Parses the input string into a 2D grid of heat loss values.
    fn parse_input(input: &str) -> Vec<Vec<usize>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
            .collect()
    }

    /// Retrieves the heat loss value for a given point in the grid.
    fn get_value(grid: &[Vec<usize>], point: Point) -> Option<usize> {
        grid.get(point.y).and_then(|row| row.get(point.x).copied())
    }

    /// Calculates the Manhattan distance between two points.
    /// Used as a heuristic for estimating the remaining cost to the goal.
    fn manhattan_distance(p1: Point, p2: Point) -> usize {
        (p1.x as isize - p2.x as isize).unsigned_abs() + (p1.y as isize - p2.y as isize).unsigned_abs()
    }

    /// Solves the least heat loss problem using a modified A* algorithm.
    pub fn least_heat_loss(input: &str) -> usize {
        let grid = parse_input(input);
        let grid_height = grid.len();
        let grid_width = grid.first().map_or(0, Vec::len);
        let mut visited = HashSet::new(); // Tracks visited states to avoid revisiting.
    
        // Define the first starting state (moving to the right)
        let start_state_right = State {
            current: Point { x: 0, y: 0 },
            direction: Direction::Right,
            direction_count: 0,
        };
        visited.insert(start_state_right); // Insert the first starting state
    
        // Define the second starting state (moving down)
        let start_state_down = State {
            current: Point { x: 0, y: 0 },
            direction: Direction::Down,
            direction_count: 0,
        };
        visited.insert(start_state_down); // Insert the second starting state
    
        let mut heap = BinaryHeap::new(); // Priority queue for exploring paths.
    
        // Push both starting states into the heap
        heap.push(Path {
            state: start_state_right,
            cost: 0,
            heuristic: manhattan_distance(start_state_right.current, Point { x: grid_width - 1, y: grid_height - 1 }),
        });
        heap.push(Path {
            state: start_state_down,
            cost: 0,
            heuristic: manhattan_distance(start_state_down.current, Point { x: grid_width - 1, y: grid_height - 1 }),
        });
    
        while let Some(current_path) = heap.pop() {
            if current_path.state.current.x == grid_width - 1
                && current_path.state.current.y == grid_height - 1
                && current_path.state.direction_count >= 4
            {
                // Termination condition: Reached the goal.
                return current_path.cost;
            }
    
            // Generate and explore next possible paths.
            current_path.next_paths(&grid, Point { x: grid_width - 1, y: grid_height - 1 })
                .into_iter()
                .filter(|path| visited.insert(path.state))
                .for_each(|path| heap.push(path));
        }
    
        usize::MAX // No valid path found.
    }
    
}
