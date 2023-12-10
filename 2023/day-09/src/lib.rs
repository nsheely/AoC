pub mod part1 {
    // Parses the input string into a vector of vectors of i64.
    // Each line of the input string is split into whitespace-separated values,
    // which are parsed into integers and collected into a vector.
    fn parse_input(input: &str) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect()
    }

    // Generates a modified Pascal's Triangle, which is used to calculate 
    // the differences between the values in the input data.
    // The triangle is a 2D vector where each row represents a level in the triangle,
    // and negative values are introduced at every second step to adjust the calculations.
    fn generate_triangle(max_size: usize) -> Vec<Vec<i64>> {
        let mut triangle = vec![vec![1]];
        for i in 0..max_size {
            let mut next_row = vec![1];
            next_row.extend(triangle[i].windows(2).map(|w| w[0] + w[1]));
            next_row.push(1);
            triangle.push(next_row);
        }
        for (row, row_values) in triangle.iter_mut().enumerate().take(max_size + 1) {
            for col in (0..=row).step_by(2) {
                row_values[col] *= -1;
            }
        }
        triangle
    }

    // Calculates the sum for a single line of input using the modified Pascal's Triangle.
    // This function extrapolates the next value in the input line's history
    // by applying the logic of the problem statement.
    fn calculate_line_sum(line: &[i64], triangle: &[Vec<i64>]) -> i64 {
        let mut sum = 0;
        let row = line.len();
        for (col, &n) in line.iter().enumerate() {
            sum += triangle[row][col] * n;
        }
        sum * if row % 2 == 0 { 1 } else { -1 }
    }

    // The main function to extrapolate and sum all lines of the input.
    // It parses the input, generates the modified Pascal's Triangle,
    // and then applies the logic to each line to calculate the final sum.
    pub fn extrapolate_and_sum(input: &str) -> i64 {
        let nums = parse_input(input);
        let max_len = nums.iter().map(Vec::len).max().unwrap_or(0);
        let triangle = generate_triangle(max_len);

        nums.into_iter()
            .map(|line| calculate_line_sum(&line, &triangle))
            .sum()
    }
}

pub mod part2 {
    // Parses the input string into a vector of vectors of i64.
    // Each line of the input string is split into whitespace-separated values,
    // which are parsed into integers and collected into a vector.
    fn parse_input(input: &str) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect()
    }

    // Generates a modified Pascal's Triangle, which is used to calculate 
    // the differences between the values in the input data.
    // The triangle is a 2D vector where each row represents a level in the triangle,
    // and negative values are introduced at every second step to adjust the calculations.
    fn generate_triangle(max_size: usize) -> Vec<Vec<i64>> {
        let mut triangle = vec![vec![1]];
        for i in 0..max_size {
            let mut next_row = vec![1];
            next_row.extend(triangle[i].windows(2).map(|w| w[0] + w[1]));
            next_row.push(1);
            triangle.push(next_row);
        }
        for (row, row_values) in triangle.iter_mut().enumerate().take(max_size + 1) {
            for col in (0..=row).step_by(2) {
                row_values[col] *= -1;
            }
        }
        triangle
    }

    // Calculates the sum for a single line of input using the modified Pascal's Triangle.
    // This function extrapolates the previous value in the input line's history
    // by applying the logic of the problem statement in reverse.
    fn calculate_line_sum(line: &[i64], triangle: &[Vec<i64>]) -> i64 {
        let mut sum = 0;
        let row = line.len();
        for (col, &n) in line.iter().enumerate() {
            sum += triangle[row][col] * n;
        }
        sum * if row % 2 == 0 { 1 } else { -1 }
    }

    // The main function to extrapolate in reverse and sum all lines of the input.
    // It parses the input, generates the modified Pascal's Triangle,
    // and then applies the logic in reverse to each line to calculate the sum of previous values.
    pub fn extrapolate_and_sum(input: &str) -> i64 {
        let nums = parse_input(input);
        let max_len = nums.iter().map(Vec::len).max().unwrap_or(0);
        let triangle = generate_triangle(max_len);

        nums.into_iter()
            .map(|mut line| {
                line.reverse(); // Reverse the line to extrapolate in reverse
                calculate_line_sum(&line, &triangle)
            })
            .sum()
    }
}

