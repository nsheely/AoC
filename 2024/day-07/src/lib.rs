pub mod part1 {
    use rayon::prelude::*;

    const MAX_OPERANDS: usize = 10;

    /// Calculate the total calibration result using reverse recursion for efficiency.
    pub fn total_calibration_result(input: &str) -> u64 {
        input
            .par_lines()
            .filter_map(|line| {
                let bytes = line.as_bytes();
                let (target, operands, len) = parse_line(bytes)?;
                if is_target_possible(target, &operands, len) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Parse a line into a target and list of operands using a fixed-size array.
    fn parse_line(line: &[u8]) -> Option<(u64, [u64; MAX_OPERANDS], usize)> {
        let colon_pos = line.iter().position(|&b| b == b':')?;
        let target = parse_u64(&line[..colon_pos])?;

        let mut operands = [0u64; MAX_OPERANDS];
        let mut len = 0;

        for chunk in line[colon_pos + 2..].split(|&b| b == b' ') {
            if len == MAX_OPERANDS {
                return None; // Exceeded maximum size
            }
            operands[len] = parse_u64(chunk)?;
            len += 1;
        }

        Some((target, operands, len))
    }

    /// Parse a `u64` directly from a slice of bytes without intermediate allocations.
    fn parse_u64(slice: &[u8]) -> Option<u64> {
        let mut result = 0u64;
        for &b in slice {
            if b < b'0' || b > b'9' {
                return None;
            }
            result = result * 10 + (b - b'0') as u64;
        }
        Some(result)
    }

    /// Determine if the target can be achieved using reverse recursion.
    fn is_target_possible(target: u64, operands: &[u64], len: usize) -> bool {
        reverse_recurse(target, operands, len)
    }

    /// Recursive function to validate the target using reverse operations.
    fn reverse_recurse(accum: u64, operands: &[u64], length: usize) -> bool {
        if length == 1 {
            return accum == operands[0];
        }

        let current = operands[length - 1];

        // Try subtraction
        if accum > current && reverse_recurse(accum - current, operands, length - 1) {
            return true;
        }

        // Try division
        accum % current == 0 && reverse_recurse(accum / current, operands, length - 1)
    }
}

pub mod part2 {
    use rayon::prelude::*;

    const MAX_OPERANDS: usize = 10;
    const POWERS_OF_10: [u64; 20] = [
        1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
        10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000,
        1_000_000_000_000_000, 10_000_000_000_000_000, 100_000_000_000_000_000,
        1_000_000_000_000_000_000, 10_000_000_000_000_000_000,
    ];

    /// Calculate the total calibration result using reverse recursion for efficiency.
    pub fn total_calibration_result(input: &str) -> u64 {
        input
            .par_lines()
            .filter_map(|line| {
                let bytes = line.as_bytes();
                let (target, operands, len) = parse_line(bytes)?;
                if is_target_possible(target, &operands, len, false) {
                    Some(target)
                } else if is_target_possible(target, &operands, len, true) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Parse a line into a target and list of operands using a fixed-size array.
    fn parse_line(line: &[u8]) -> Option<(u64, [u64; MAX_OPERANDS], usize)> {
        let colon_pos = line.iter().position(|&b| b == b':')?;
        let target = parse_u64(&line[..colon_pos])?;

        let mut operands = [0u64; MAX_OPERANDS];
        let mut len = 0;

        for chunk in line[colon_pos + 2..].split(|&b| b == b' ') {
            if len == MAX_OPERANDS {
                return None; // Exceeded maximum size
            }
            operands[len] = parse_u64(chunk)?;
            len += 1;
        }

        Some((target, operands, len))
    }

    /// Parse a `u64` directly from a slice of bytes without intermediate allocations.
    fn parse_u64(slice: &[u8]) -> Option<u64> {
        let mut result = 0u64;
        for &b in slice {
            if b < b'0' || b > b'9' {
                return None;
            }
            result = result * 10 + (b - b'0') as u64;
        }
        Some(result)
    }

    /// Determine if the target can be achieved using reverse recursion.
    fn is_target_possible(target: u64, operands: &[u64], len: usize, allow_concat: bool) -> bool {
        reverse_recurse(target, operands, len, allow_concat)
    }

    /// Recursive function to validate the target using reverse operations.
    fn reverse_recurse(accum: u64, operands: &[u64], length: usize, allow_concat: bool) -> bool {
        if length == 1 {
            return accum == operands[0];
        }

        let current = operands[length - 1];

        // Try subtraction
        if accum > current && reverse_recurse(accum - current, operands, length - 1, allow_concat) {
            return true;
        }

        // Try division
        if accum % current == 0 && reverse_recurse(accum / current, operands, length - 1, allow_concat)
        {
            return true;
        }

        // Try concatenation
        if allow_concat {
            let factor = POWERS_OF_10[(current.ilog10() as usize) + 1];
            if accum % factor == current
                && reverse_recurse(accum / factor, operands, length - 1, allow_concat)
            {
                return true;
            }
        }

        false
    }
}
