pub mod part1 {
    use rayon::prelude::*;

    // Part 1: Find the Reflection Line

    // Calculate the sum of reflection line positions for all patterns in the input.
    pub fn sum_of_pattern_notes(input: &str) -> u32 {
        // Split the input into separate patterns using "\n\n" as the delimiter.
        let grids: Vec<&str> = input.split("\n\n").collect();

        // Use Rayon's parallel iterator to process patterns concurrently.
        grids.par_iter()
             .map(|&grid| {
                 // Convert the pattern into binary representations (horizontal and vertical).
                 let (horizontal, vertical) = convert_grid_to_binary(grid);

                 // Calculate the reflection score for this pattern.
                 find_reflection_score(&horizontal, &vertical)
             })
             .sum() // Sum the reflection scores from all patterns.
    }

    // Converts a pattern grid into binary representations (horizontal and vertical).
    fn convert_grid_to_binary(grid: &str) -> (Vec<u32>, Vec<u32>) {
        // Split the pattern into lines.
        let lines: Vec<&str> = grid.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        // Initialize vectors to store the binary representation.
        let mut horizontal = vec![0; num_rows];
        let mut vertical = vec![0; num_cols];

        // Iterate through the characters in the pattern and set corresponding bits in the binary vectors.
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    // Set the corresponding bit in the binary representation.
                    horizontal[i] |= 1 << j;
                    vertical[j] |= 1 << i;
                }
            }
        }

        // Return the binary representations.
        (horizontal, vertical)
    }

    // Calculates the reflection score by summing the positions of reflection lines for horizontal and vertical reflections.
    fn find_reflection_score(horizontal: &[u32], vertical: &[u32]) -> u32 {
        // Calculate the reflection score for horizontal reflection and multiply it by 100.
        check_reflections(horizontal) * 100 + check_reflections(vertical)
    }

    // Checks for reflection points in the binary representation.
    // It returns the position (line/column number) of the reflection point or 0 if no reflection point is found.
    fn check_reflections(arr: &[u32]) -> u32 {
        for line in 0..arr.len() {
            // Check if the current line is a reflection point.
            if is_reflection_point(arr, line) {
                return (line + 1).try_into().unwrap();
            }
        }
        0
    }

    // Checks if the given position (line/column) is a reflection point.
    // A reflection point occurs when the binary values on both sides of the mid-point are the same.
    // This function returns true if it's a reflection point and false otherwise.
    fn is_reflection_point(arr: &[u32], mid: usize) -> bool {
        let mut left: i32 = mid as i32;
        let mut right = mid + 1;
    
        // Compare binary values from left and right, moving towards the edges.
        while left > 0 && right < arr.len() - 1 && arr[left as usize] == arr[right] {
            left = left.saturating_sub(1);
            right += 1;
        }
    
        // Check if the reflection point is at the edge (no match on one side) and the values are the same.
        (left == 0 || right == arr.len() - 1) && arr[left as usize] == arr[right]
    }
}

pub mod part2 {
    use rayon::prelude::*;

    // Part 2: Fix the Smudges

    // Calculate the sum of reflection line positions for all patterns in the input, 
    // taking into account smudges (incorrect characters).
    pub fn sum_of_pattern_notes(input: &str) -> u32 {
        // Split the input into separate patterns using "\n\n" as the delimiter.
        let grids: Vec<&str> = input.split("\n\n").collect();

        // Use Rayon's parallel iterator to process patterns concurrently.
        grids.par_iter()
             .map(|&grid| {
                 // Convert the pattern into binary representations (horizontal and vertical).
                 let (horizontal, vertical) = convert_grid_to_binary(grid);

                 // Calculate the reflection score for this pattern, considering smudges.
                 find_reflection_score(&horizontal, &vertical)
             })
             .sum() // Sum the reflection scores from all patterns.
    }

    // Converts a pattern grid into binary representations (horizontal and vertical).
    fn convert_grid_to_binary(grid: &str) -> (Vec<u32>, Vec<u32>) {
        // Split the pattern into lines.
        let lines: Vec<&str> = grid.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        // Initialize vectors to store the binary representation.
        let mut horizontal = vec![0; num_rows];
        let mut vertical = vec![0; num_cols];

        // Iterate through the characters in the pattern and set corresponding bits in the binary vectors.
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    // Set the corresponding bit in the binary representation.
                    horizontal[i] |= 1 << j;
                    vertical[j] |= 1 << i;
                }
            }
        }

        // Return the binary representations.
        (horizontal, vertical)
    }

    // Calculates the reflection score by summing the positions of reflection lines for horizontal and vertical reflections,
    // considering smudges.
    fn find_reflection_score(horizontal: &[u32], vertical: &[u32]) -> u32 {
        // Calculate the reflection score for horizontal reflection and multiply it by 100.
        check_reflections(horizontal) * 100 + check_reflections(vertical)
    }

    // Checks for reflection points in the binary representation, considering smudges.
    // It returns the position (line/column number) of the reflection point or 0 if no reflection point is found.
    fn check_reflections(arr: &[u32]) -> u32 {
        for line in 0..arr.len() {
            // Check if the current line is a reflection point with a smudge.
            if is_smudged_reflection_point(arr, line) {
                return (line + 1).try_into().unwrap();
            }
        }
        0
    }

    // Checks if the given position (line/column) is a reflection point with a smudge.
    // A reflection point with a smudge occurs when there is exactly one difference (smudge)
    // between the binary values on both sides of the mid-point.
    // This function returns true if it's a reflection point with one smudge and false otherwise.
    fn is_smudged_reflection_point(arr: &[u32], mid: usize) -> bool {
        let mut left: i32 = mid as i32;
        let mut right = mid + 1;
        let mut smudges = 0;
    
        // Compare binary values from left and right, moving towards the edges, and count smudges.
        while left >= 0 && right < arr.len() {
            smudges += (arr[left as usize] ^ arr[right]).count_ones();
            if smudges > 1 {
                return false;
            }
            left = left.saturating_sub(1);
            right += 1;
        }
        // Check if the reflection point is at the edge (no match on one side) and there is exactly one smudge.
        (left <= 0 || right == arr.len()) && smudges == 1
    }
}
