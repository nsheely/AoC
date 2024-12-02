pub mod part1 {
    pub fn safe_reports(input: &str) -> u32 {
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        let mut safe_count = 0u32;
        // Fixed array for storing numbers in each line, avoids heap allocations
        let mut levels = [0i32; 16];

        unsafe {
            // Process each line in the input
            while i < len {
                let mut num_count = 0;
                
                // Parse all numbers in the current line until we hit a newline
                while i < len && *bytes.get_unchecked(i) != b'\n' {
                    // Skip over any spaces between numbers
                    while i < len && *bytes.get_unchecked(i) == b' ' {
                        i += 1;
                    }
                    
                    // Convert ASCII digits to a single number (e.g., "123" -> 123)
                    let mut num = 0i32;
                    while i < len && bytes.get_unchecked(i).is_ascii_digit() {
                        num = num * 10 + (*bytes.get_unchecked(i) - b'0') as i32;
                        i += 1;
                    }
                    
                    // Store the parsed number in our levels array
                    *levels.get_unchecked_mut(num_count) = num;
                    num_count += 1;
                }
                i += 1; // Move past the newline
                
                // Only process sequences with at least 2 numbers
                if num_count >= 2 {
                    // Check the first two numbers to determine if sequence is increasing/decreasing
                    let prev = *levels.get_unchecked(0);
                    let next = *levels.get_unchecked(1);
                    let increasing = next > prev;
                    
                    // First check if the initial difference is valid (≤3)
                    if (next - prev).abs() <= 3 {
                        let mut prev = prev;
                        let mut is_safe = true;
                        
                        // Check each subsequent number in the sequence
                        for idx in 1..num_count {
                            let num = *levels.get_unchecked(idx);
                            let diff = num - prev;
                            // A sequence is unsafe if:
                            // - The difference between numbers is > 3
                            // - An increasing sequence has a decrease
                            // - A decreasing sequence has an increase
                            if diff.abs() > 3 || (increasing && diff <= 0) || (!increasing && diff >= 0) {
                                is_safe = false;
                                break;
                            }
                            prev = num;
                        }
                        
                        safe_count += is_safe as u32;
                    }
                }
            }
        }
        safe_count
    }
}

pub mod part2 {
    pub fn safe_reports(input: &str) -> u32 {
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        let mut safe_count = 0u32;
        let mut levels = [0i32; 16];
    
        unsafe {
            while i < len {
                let mut num_count = 0;
                
                while i < len && *bytes.get_unchecked(i) != b'\n' {
                    while i < len && *bytes.get_unchecked(i) == b' ' { i += 1; }
                    let mut num = 0i32;
                    while i < len && bytes.get_unchecked(i).is_ascii_digit() {
                        num = num * 10 + (*bytes.get_unchecked(i) - b'0') as i32;
                        i += 1;
                    }
                    if num_count < 16 {
                        *levels.get_unchecked_mut(num_count) = num;
                        num_count += 1;
                    }
                }
                i += 1;
                
                if num_count < 2 { continue; }
    
                // For each position, try removing that number and check if remaining sequence is valid
                'outer: for skip in 0..num_count {
                    let mut prev = if skip == 0 { *levels.get_unchecked(1) } else { *levels.get_unchecked(0) };
                    let mut curr_idx = if skip <= 1 { 2 } else { 1 };
                    let direction = if skip == 0 { 
                        *levels.get_unchecked(2) - *levels.get_unchecked(1) > 0 
                    } else { 
                        *levels.get_unchecked(1) - *levels.get_unchecked(0) > 0 
                    };
    
                    while curr_idx < num_count {
                        if curr_idx == skip { 
                            curr_idx += 1;
                            continue;
                        }
                        let curr = *levels.get_unchecked(curr_idx);
                        let diff = curr - prev;
                        if diff.abs() > 3 || (direction && diff <= 0) || (!direction && diff >= 0) {
                            continue 'outer;
                        }
                        prev = curr;
                        curr_idx += 1;
                    }
                    safe_count += 1;
                    break;
                }
            }
        }
        safe_count
    }
}
