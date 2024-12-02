pub mod part1 {
    pub fn distance_between_lists(input: &str) -> u32 {
        let mut left = Vec::with_capacity(1024);
        let mut right = Vec::with_capacity(1024);
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        // Parsing numbers
        while i < len {
            // Parse left number
            let mut num = 0u32;
            while i < len {
                let digit = bytes[i];
                if digit.is_ascii_digit() {
                    num = num * 10 + (digit - b'0') as u32;
                    i += 1;
                } else {
                    break;
                }
            }
            left.push(num);
            i += 1; // Skip space or newline

            // Parse right number
            num = 0u32;
            while i < len {
                let digit = bytes[i];
                if digit.is_ascii_digit() {
                    num = num * 10 + (digit - b'0') as u32;
                    i += 1;
                } else {
                    break;
                }
            }
            right.push(num);

            // Skip newline or carriage return
            while i < len && (bytes[i] == b'\n' || bytes[i] == b'\r' || bytes[i] == b' ') {
                i += 1;
            }
        }

        left.sort_unstable();
        right.sort_unstable();

        // Replace closure with for loop
        let mut total_distance = 0u32;
        let n = left.len();
        for idx in 0..n {
            total_distance += left[idx].abs_diff(right[idx]);
        }
        total_distance
    }
}

pub mod part2 {
    pub fn similarity_score(input: &str) -> u64 {
        let mut counts = [0u32; 100_000]; // Assuming IDs are in 0..99,999
        let mut left = Vec::with_capacity(1024);
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        // Parsing numbers
        while i < len {
            // Parse left number
            let mut num = 0usize;
            while i < len {
                let digit = bytes[i];
                if digit.is_ascii_digit() {
                    num = num * 10 + (digit - b'0') as usize;
                    i += 1;
                } else {
                    break;
                }
            }
            left.push(num);
            i += 1; // Skip space or newline

            // Parse right number
            num = 0usize;
            while i < len {
                let digit = bytes[i];
                if digit.is_ascii_digit() {
                    num = num * 10 + (digit - b'0') as usize;
                    i += 1;
                } else {
                    break;
                }
            }
            counts[num] += 1;

            // Skip newline or carriage return
            while i < len && (bytes[i] == b'\n' || bytes[i] == b'\r' || bytes[i] == b' ') {
                i += 1;
            }
        }

        // Replace closure with for loop
        let mut similarity_score = 0u64;
        for &num in &left {
            // Use unsafe to eliminate bounds checks
            unsafe {
                similarity_score += num as u64 * *counts.get_unchecked(num) as u64;
            }
        }
        similarity_score
    }
}