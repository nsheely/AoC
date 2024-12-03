pub mod part1 {
    #[inline(always)]
    fn parse_number(bytes: &[u8], max_len: usize) -> Option<(u32, usize)> {
        let len = bytes.len().min(max_len);
        let mut val = 0u32;
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            if b'0' <= b && b <= b'9' {
                val = val * 10 + (b - b'0') as u32;
                i += 1;
            } else {
                break;
            }
        }
        if i == 0 {
            None
        } else {
            Some((val, i))
        }
    }

    pub fn sum_mul_results(corrupted_memory: &str) -> u32 {
        let bytes = corrupted_memory.as_bytes();
        let len = bytes.len();
        let mut total_sum = 0u32;
        let mut i = 0;

        while i < len {
            // Look for 'm' character
            if bytes[i] == b'm' {
                // Check if the next characters are 'ul('
                if i + 3 < len && &bytes[i..i + 4] == b"mul(" {
                    let mut j = i + 4;

                    // Parse first number (X)
                    if let Some((x, consumed_x)) = parse_number(&bytes[j..], 3) {
                        j += consumed_x;

                        // Expect a comma
                        if j < len && bytes[j] == b',' {
                            j += 1;

                            // Parse second number (Y)
                            if let Some((y, consumed_y)) = parse_number(&bytes[j..], 3) {
                                j += consumed_y;

                                // Expect a closing parenthesis
                                if j < len && bytes[j] == b')' {
                                    j += 1;

                                    // Valid mul instruction found
                                    total_sum += x * y;
                                    i = j;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        total_sum
    }
}


pub mod part2 {
    #[inline(always)]
    fn parse_number(bytes: &[u8], max_len: usize) -> Option<(u32, usize)> {
        let len = bytes.len().min(max_len);
        let mut val = 0u32;
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            if b'0' <= b && b <= b'9' {
                val = val * 10 + (b - b'0') as u32;
                i += 1;
            } else {
                break;
            }
        }
        if i == 0 {
            None
        } else {
            Some((val, i))
        }
    }

    pub fn sum_mul_results(corrupted_memory: &str) -> u32 {
        let bytes = corrupted_memory.as_bytes();
        let len = bytes.len();
        let mut total_sum = 0u32;
        let mut i = 0;
        let mut enabled = true; // Mul instructions are initially enabled

        while i < len {
            if bytes[i] == b'm' {
                // Check if the next characters are 'ul('
                if i + 3 < len && &bytes[i..i + 4] == b"mul(" {
                    let mut j = i + 4;

                    // Parse first number (X)
                    if let Some((x, consumed_x)) = parse_number(&bytes[j..], 3) {
                        j += consumed_x;

                        // Expect a comma
                        if j < len && bytes[j] == b',' {
                            j += 1;

                            // Parse second number (Y)
                            if let Some((y, consumed_y)) = parse_number(&bytes[j..], 3) {
                                j += consumed_y;

                                // Expect a closing parenthesis
                                if j < len && bytes[j] == b')' {
                                    j += 1;

                                    // Valid mul instruction found
                                    if enabled {
                                        total_sum += x * y;
                                    }
                                    i = j;
                                    continue;
                                }
                            }
                        }
                    }
                }
            } else if bytes[i] == b'd' {
                // Check for 'do()' or 'don't()' instructions
                if i + 3 < len && &bytes[i..i + 4] == b"do()" {
                    // 'do()' instruction
                    enabled = true;
                    i += 4; // Move index past 'do()'
                    continue;
                } else if i + 6 < len && &bytes[i..i + 7] == b"don't()" {
                    // 'don't()' instruction
                    enabled = false;
                    i += 7; // Move index past 'don't()'
                    continue;
                }
            }
            i += 1;
        }
        total_sum
    }
}

