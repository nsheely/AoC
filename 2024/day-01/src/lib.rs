pub mod part1 {
    pub fn distance_between_lists(input: &str) -> u32 {
        let mut left = Vec::with_capacity(1024);
        let mut right = Vec::with_capacity(1024);
        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut num = 0;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }
            left.push(num);
            i += 1;

            num = 0;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }
            right.push(num);

            if i < bytes.len() {
                i += 1;
            }
        }

        left.sort_unstable();
        right.sort_unstable();
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }
}

pub mod part2 {
    pub fn similarity_score(input: &str) -> u64 {
        let mut counts = [0u32; 100_000];  // 5 digit numbers (0-99999)
        let mut left = Vec::with_capacity(1024);
        let bytes = input.as_bytes();
        let mut i = 0;
        
        // First pass: collect left numbers and count right numbers
        while i < bytes.len() {
            // Parse and store left number
            let mut num = 0;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            left.push(num);
            i += 1;
            
            // Parse and count right number
            num = 0;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            counts[num] += 1;
            
            if i < bytes.len() { i += 1; }
        }
        
        // Second pass: calculate similarity score
        left.iter()
            .map(|&num| num as u64 * counts[num] as u64)
            .sum()
    }
}