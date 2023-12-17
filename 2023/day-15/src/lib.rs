pub mod part1 {
    pub fn compute_hash(input: &str) -> usize {
        input.split(',').map(hash_step).sum()
    }

    fn hash_step(step: &str) -> usize {
        step.as_bytes().iter().fold(0, |acc, &b| (acc + (b as usize)) * 17 % 256)
    }
}

pub mod part2 {
    const BOXES_COUNT: usize = 256;

    pub fn compute_focusing_power(input: &str) -> usize {
        let mut boxes = make_boxes_array();
    
        for step in input.split(',') {
            let (label, operation, value) = parse_step(step);
            let box_index = hash_label(label);

            match operation {
                '-' => remove_lens(&mut boxes[box_index], label),
                '=' => insert_lens(&mut boxes[box_index], label, value),
                _ => panic!("Unexpected operation character: {}", operation),
            };
        }

        calculate_focusing_power(&boxes)
    }

    fn make_boxes_array() -> [Vec<(String, usize)>; BOXES_COUNT] {
        std::array::from_fn(|_| Vec::new())
    }    

    #[inline]
    fn hash_label(label: &str) -> usize {
        label.as_bytes().iter().fold(0, |acc, &b| (acc + (b as usize)) * 17 % 256)
    }

    #[inline]
    fn parse_step(step: &str) -> (&str, char, usize) {
        let split_index = step.find(|c: char| !c.is_alphabetic()).unwrap();
        let (label, rest) = step.split_at(split_index);
        let operation = rest.chars().next().unwrap();
        let value = if operation == '=' { rest[1..].parse().unwrap() } else { 0 };
        (label, operation, value)
    }

    #[inline]
    fn remove_lens(lenses: &mut Vec<(String, usize)>, label: &str) {
        if let Some(pos) = lenses.iter().position(|(l, _)| l == label) {
            lenses.remove(pos);
        }
    }

    #[inline]
    fn insert_lens(lenses: &mut Vec<(String, usize)>, label: &str, focal_length: usize) {
        match lenses.iter().position(|(l, _)| l == label) {
            Some(pos) => lenses[pos].1 = focal_length,
            None => lenses.push((label.to_string(), focal_length)),
        }
    }

    fn calculate_focusing_power(boxes: &[Vec<(String, usize)>; BOXES_COUNT]) -> usize {
        boxes.iter().enumerate().fold(0, |acc, (box_index, lenses)| {
            acc + lenses.iter().enumerate().map(|(slot_index, (_, focal_length))| {
                (box_index + 1) * (slot_index + 1) * focal_length
            }).sum::<usize>()
        })
    }
}
