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
        // Two fixed arrays: one for the original sequence and one for testing sequences with a number removed
        let mut levels = [0i32; 16];
        let mut temp_levels = [0i32; 16];

        unsafe {
            // Process each line in the input
            while i < len {
                let mut num_count = 0;
                
                // Parse all numbers in the current line, identical to part 1
                while i < len && *bytes.get_unchecked(i) != b'\n' {
                    while i < len && *bytes.get_unchecked(i) == b' ' {
                        i += 1;
                    }
                    
                    let mut num = 0i32;
                    while i < len && bytes.get_unchecked(i).is_ascii_digit() {
                        num = num * 10 + (*bytes.get_unchecked(i) - b'0') as i32;
                        i += 1;
                    }
                    
                    *levels.get_unchecked_mut(num_count) = num;
                    num_count += 1;
                }
                i += 1;
                
                if num_count >= 2 {
                    // First check if sequence is safe without removing any numbers
                    let prev = *levels.get_unchecked(0);
                    let next = *levels.get_unchecked(1);
                    let increasing = next > prev;
                    
                    if (next - prev).abs() <= 3 {
                        let mut prev = prev;
                        let mut is_safe = true;
                        
                        for idx in 1..num_count {
                            let num = *levels.get_unchecked(idx);
                            let diff = num - prev;
                            if diff.abs() > 3 || (increasing && diff <= 0) || (!increasing && diff >= 0) {
                                is_safe = false;
                                break;
                            }
                            prev = num;
                        }
                        
                        if is_safe {
                            safe_count += 1;
                            continue;  // If sequence is already safe, skip to next line
                        }
                    }

                    // If sequence wasn't safe, try removing each number one at a time
                    for skip_idx in 0..num_count {
                        // Create a new sequence with one number removed
                        let mut temp_count = 0;
                        for idx in 0..num_count {
                            if idx != skip_idx {
                                *temp_levels.get_unchecked_mut(temp_count) = *levels.get_unchecked(idx);
                                temp_count += 1;
                            }
                        }

                        // Check if this modified sequence is safe, using same logic as above
                        let prev = *temp_levels.get_unchecked(0);
                        let next = *temp_levels.get_unchecked(1);
                        let increasing = next > prev;
                        
                        if (next - prev).abs() <= 3 {
                            let mut prev = prev;
                            let mut is_safe = true;
                            
                            for idx in 1..temp_count {
                                let num = *temp_levels.get_unchecked(idx);
                                let diff = num - prev;
                                if diff.abs() > 3 || (increasing && diff <= 0) || (!increasing && diff >= 0) {
                                    is_safe = false;
                                    break;
                                }
                                prev = num;
                            }
                            
                            if is_safe {
                                safe_count += 1;
                                break;  // Found a valid sequence by removing one number, move to next line
                            }
                        }
                    }
                }
            }
        }
        safe_count
    }
}