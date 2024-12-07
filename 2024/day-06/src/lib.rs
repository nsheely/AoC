pub mod common {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Dir {
        N,
        E,
        S,
        W,
    }

    impl Dir {
        pub fn rotate(self) -> Dir {
            match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            }
        }

        pub fn step(self, pos: usize, line_len: usize, grid_len: usize) -> Option<usize> {
            match self {
                Dir::N => (pos >= line_len).then(|| pos - line_len),
                Dir::E => ((pos + 1) % line_len != 0).then(|| pos + 1),
                Dir::S => (pos + line_len < grid_len).then(|| pos + line_len),
                Dir::W => (pos % line_len != 0).then(|| pos - 1),
            }
        }
    }
}

pub mod part1 {
    use super::common::Dir;

    pub fn guard_positions(input: &str) -> u32 {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        let mut visited = vec![0u64; (rows * cols + 63) / 64];
        let mut visited_count = 0;

        // Find starting position
        let start_pos = input.bytes().position(|b| b == b'^').unwrap();

        let mut pos = start_pos;
        let mut dir = Dir::N;

        loop {
            // Mark current position as visited
            if mark_visited(&mut visited, pos) {
                visited_count += 1;
            }

            // Determine the next position
            if let Some(next_pos) = dir.step(pos, cols + 1, rows * (cols + 1)) {
                if input.as_bytes()[next_pos] == b'#' {
                    dir = dir.rotate(); // Turn right at an obstacle
                } else {
                    pos = next_pos; // Move forward
                }
            } else {
                break; // Exit the map
            }
        }

        visited_count
    }

    fn mark_visited(visited: &mut [u64], pos: usize) -> bool {
        let idx = pos / 64;
        let bit = 1 << (pos % 64);
        if visited[idx] & bit == 0 {
            visited[idx] |= bit;
            true
        } else {
            false
        }
    }
}

pub mod part2 {
    use super::common::Dir;
    use std::collections::HashSet;

    pub fn count_guard_loop_additions(input: &str) -> u32 {
        let line_len = input.lines().next().unwrap().len() + 1; // Include newline
        let bytes = input.as_bytes();
        let grid_len = bytes.len();

        // Find the starting position
        let start_pos = bytes.iter().position(|&b| b == b'^').unwrap();

        let mut possible_obstacles = 0;

        for pos in 0..grid_len {
            // Skip invalid positions: walls, starting position, and newlines
            if bytes[pos] == b'#' || pos == start_pos || bytes[pos] == b'\n' {
                continue;
            }

            // Simulate guard's movement with the new obstacle in place
            if causes_loop(bytes, line_len, grid_len, start_pos, pos) {
                possible_obstacles += 1;
            }
        }

        possible_obstacles
    }

    fn causes_loop(
        bytes: &[u8],
        line_len: usize,
        grid_len: usize,
        start_pos: usize,
        obstacle_pos: usize,
    ) -> bool {
        let mut visited_states = HashSet::new();
        let mut pos = start_pos;
        let mut dir = Dir::N;

        loop {
            // Track the state (position and direction)
            let state = (pos, dir);
            if !visited_states.insert(state) {
                return true; // Loop detected
            }

            // Determine the next position and direction
            if let Some(next_pos) = dir.step(pos, line_len, grid_len) {
                if next_pos == obstacle_pos || bytes[next_pos] == b'#' {
                    dir = dir.rotate(); // Turn right at an obstacle
                } else {
                    pos = next_pos; // Move forward
                }
            } else {
                return false; // Exit the map
            }
        }
    }
}
