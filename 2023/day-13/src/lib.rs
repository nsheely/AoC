pub mod part1 {
    use rayon::prelude::*;
    pub fn sum_of_pattern_notes(input: &str) -> u32 {
        let grids: Vec<&str> = input.split("\n\n").collect();
        grids.par_iter()
             .map(|&grid| {
                 let (horizontal, vertical) = convert_grid_to_binary(grid);
                 find_reflection_score(&horizontal, &vertical)
             })
             .sum()
    }

    fn convert_grid_to_binary(grid: &str) -> (Vec<u32>, Vec<u32>) {
        let lines: Vec<&str> = grid.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        let mut horizontal = vec![0; num_rows];
        let mut vertical = vec![0; num_cols];

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    horizontal[i] |= 1 << j;
                    vertical[j] |= 1 << i;
                }
            }
        }

        (horizontal, vertical)
    }

    fn find_reflection_score(horizontal: &[u32], vertical: &[u32]) -> u32 {
        check_reflections(horizontal) * 100 + check_reflections(vertical)
    }

    fn check_reflections(arr: &[u32]) -> u32 {
        for line in 0..arr.len() {
            if is_reflection_point(arr, line) {
                return (line + 1).try_into().unwrap();
            }
        }
        0
    }

    fn is_reflection_point(arr: &[u32], mid: usize) -> bool {
        let mut left: i32 = mid as i32;
        let mut right = mid + 1;
    
        while left > 0 && right < arr.len() - 1 && arr[left as usize] == arr[right] {
            left = left.saturating_sub(1);
            right += 1;
        }
    
        (left == 0 || right == arr.len() - 1) && arr[left as usize] == arr[right]
    }
}

pub mod part2 {
    use rayon::prelude::*;
    pub fn sum_of_pattern_notes(input: &str) -> u32 {
        let grids: Vec<&str> = input.split("\n\n").collect();
        grids.par_iter()
             .map(|&grid| {
                 let (horizontal, vertical) = convert_grid_to_binary(grid);
                 find_reflection_score(&horizontal, &vertical)
             })
             .sum()
    }

    fn convert_grid_to_binary(grid: &str) -> (Vec<u32>, Vec<u32>) {
        let lines: Vec<&str> = grid.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        let mut horizontal = vec![0; num_rows];
        let mut vertical = vec![0; num_cols];

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    horizontal[i] |= 1 << j;
                    vertical[j] |= 1 << i;
                }
            }
        }

        (horizontal, vertical)
    }

    fn find_reflection_score(horizontal: &[u32], vertical: &[u32]) -> u32 {
        check_reflections(horizontal) * 100 + check_reflections(vertical)
    }

    fn check_reflections(arr: &[u32]) -> u32 {
        for line in 0..arr.len() {
            if is_smudged_reflection_point(arr, line) {
                return (line + 1).try_into().unwrap();
            }
        }
        0
    }

    fn is_smudged_reflection_point(arr: &[u32], mid: usize) -> bool {
        let mut left: i32 = mid as i32;
        let mut right = mid + 1;
        let mut smudges = 0;
    
        while left >= 0 && right < arr.len() {
            smudges += (arr[left as usize] ^ arr[right]).count_ones();
            if smudges > 1 {
                return false;
            }
            left = left.saturating_sub(1);
            right += 1;
        }
        (left <= 0 || right == arr.len()) && smudges == 1
    }
}
