pub mod part1 {
    use std::collections::HashMap;

    // A struct representing a single workflow rule.
    // It holds a reference to a target workflow name and an optional condition.
    #[derive(Debug, Clone)]
    struct WorkflowRule<'a>(&'a str, Option<(usize, u8, i16)>);

    // Parses the input string into a collection of workflows and part ratings.
    fn parse(input: &str) -> (HashMap<&str, Vec<WorkflowRule<'_>>>, Vec<Vec<i16>>) {
        // Split the input into two sections: workflows and ratings.
        let (workflows_section, ratings_section) = input.split_once("\n\n").unwrap();
        // Parse these sections separately.
        let workflows = parse_workflows(workflows_section);
        let ratings = parse_ratings(ratings_section);
        (workflows, ratings)
    }

    // Parses the workflows section and returns a map of workflow names to their rules.
    fn parse_workflows(workflows_str: &str) -> HashMap<&str, Vec<WorkflowRule<'_>>> {
        workflows_str
            .lines()
            .map(|line| {
                let (workflow_name, rules) = line.split_once('{').unwrap();
                let rules = rules[..rules.len() - 1]
                    .split(',')
                    .map(|rule| {
                        if let Some((condition, target)) = rule.split_once(':') {
                            // Parses the condition and target of a rule.
                            let condition_byte = condition.as_bytes()[0];
                            let condition_position = match condition_byte {
                                b'x' => 0,
                                b'm' => 1,
                                b'a' => 2,
                                b's' => 3,
                                _ => unreachable!(), // This should never happen if input is well-formed.
                            };
                            WorkflowRule(
                                target,
                                Some((condition_position, condition.as_bytes()[1], condition[2..].parse().unwrap())),
                            )
                        } else {
                            // Rules without a condition just have a target workflow.
                            WorkflowRule(rule, None)
                        }
                    })
                    .collect();
                (workflow_name, rules)
            })
            .collect()
    }

    // Parses the part ratings section into a vector of vectors.
    fn parse_ratings(ratings_str: &str) -> Vec<Vec<i16>> {
        ratings_str
            .lines()
            .map(|line| {
                line[1..line.len() - 1] // Removes curly braces.
                    .split(',')
                    .map(|rating| rating[2..].parse().unwrap()) // Parses each rating value.
                    .collect()
            })
            .collect()
    }

    // Determines if a part is accepted according to the workflow rules.
    fn is_part_accepted<'a>(
        workflows: &'a HashMap<&str, Vec<WorkflowRule<'a>>>,
        part_ratings: &[i16],
    ) -> bool {
        // Calls `process_workflow` and checks if the result equals 1, indicating acceptance.
        process_workflow(
            workflows,
            "in",
            [
                (part_ratings[0], part_ratings[0] + 1),
                (part_ratings[1], part_ratings[1] + 1),
                (part_ratings[2], part_ratings[2] + 1),
                (part_ratings[3], part_ratings[3] + 1),
            ],
        ) == 1
    }

    // Processes the workflow recursively and returns the total rating of accepted parts.
    fn process_workflow<'a>(
        workflows: &'a HashMap<&str, Vec<WorkflowRule<'a>>>,
        initial_workflow: &'a str,
        initial_possible_ranges: [(i16, i16); 4],
    ) -> i64 {
        let mut stack = vec![(initial_workflow, initial_possible_ranges)];
        let mut total = 0;

        // Iteratively process the workflow.
        while let Some((current_workflow, possible_ranges)) = stack.pop() {
            match current_workflow {
                "A" => {
                    // If 'A' is reached, calculate the product of the possible ranges.
                    total += possible_ranges
                        .iter()
                        .map(|(lower, upper)| i64::from(upper - lower))
                        .product::<i64>();
                }
                "R" => continue, // Skip 'R' as it represents rejection.
                _ => {
                    // For other workflow rules, check conditions and process further.
                    if let Some(rules) = workflows.get(current_workflow) {
                        for rule in rules {
                            match rule {
                                WorkflowRule(target_workflow, None) => {
                                    // Push the target workflow if no condition is specified.
                                    stack.push((target_workflow, possible_ranges));
                                    break;
                                }
                                WorkflowRule(target_workflow, Some((index, operator, limit))) => {
                                    let (lower, upper) = possible_ranges[*index];
                                    if (operator == &b'<' && upper <= *limit) || (operator == &b'>' && lower > *limit) {
                                        // Push the target workflow if the condition is met.
                                        stack.push((target_workflow, possible_ranges));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        total
    }

    // Calculates the sum of ratings for all parts that are accepted by the workflows.
    pub fn calculate_accepted_parts_sum(input: &str) -> i64 {
        let (workflows, ratings) = parse(input);
        ratings
            .into_iter()
            .filter_map(|part_ratings| {
                // For each set of part ratings, check if it's accepted and sum up their ratings.
                is_part_accepted(&workflows, &part_ratings).then(|| {
                    part_ratings
                        .iter()
                        .map(|&rating| i64::from(rating))
                        .sum::<i64>()
                })
            })
            .sum()
    }
}
pub mod part2 {
    use std::collections::HashMap;

    // Represents a single workflow rule, holding a target workflow name and an optional condition.
    #[derive(Debug)]
    struct WorkflowRule<'a>(&'a str, Option<(usize, u8, i16)>);

    // Parses the input string into a collection of workflows.
    // Returns a map of workflow names to their associated rules.
    fn parse_to_workflows(input: &str) -> HashMap<&str, Vec<WorkflowRule<'_>>> {
        let (workflows, _) = input.split_once("\n\n").unwrap();
        workflows
            .lines()
            .map(|line| {
                let (name, rules) = line.split_once('{').unwrap();
                let rules = rules[..rules.len() - 1]
                    .split(',')
                    .map(|rule| {
                        // Parse each rule, splitting it into a condition and a target.
                        if let Some((condition, target)) = rule.split_once(':') {
                            let condition_byte = condition.as_bytes()[0];
                            let condition_position = match condition_byte {
                                b'x' => 0, b'm' => 1, b'a' => 2, b's' => 3,
                                _ => unreachable!(), // Ensures well-formed input.
                            };
                            WorkflowRule(
                                target,
                                Some((condition_position, condition.as_bytes()[1], condition[2..].parse().unwrap())),
                            )
                        } else {
                            // If no condition is specified, only the target is considered.
                            WorkflowRule(rule, None)
                        }
                    })
                    .collect();
                (name, rules)
            })
            .collect()
    }

    // Processes the workflow rules iteratively and calculates the total accepted combinations.
    fn process_rule(
        workflows: &HashMap<&str, Vec<WorkflowRule<'_>>>,
        rule: &str,
        mut possible: [(i16, i16); 4],
    ) -> i64 {
        match rule {
            "A" => {
                // If 'A' is reached, calculate the product of the possible ranges.
                return possible
                    .into_iter()
                    .map(|(l, h)| i64::from(h - l))
                    .product::<i64>()
            }
            "R" => return 0, // Ignore 'R' as it represents rejection.
            _ => (),
        }
        let mut total = 0;
        // Iterate through the rules to evaluate conditions and compute totals.
        for WorkflowRule(target, condition) in &workflows[&rule] {
            let short = |target| total + process_rule(workflows, target, possible);
            match *condition {
                None => return short(target), // No condition means direct transition to the target.
                Some((idx, op, limit)) => match (possible[idx], op) {
                    // Evaluate each condition and update totals and ranges accordingly.
                    ((_, u), b'<') if u <= limit => return short(target),
                    ((l, _), b'>') if l > limit => return short(target),
                    ((l, u), b'<') if l < limit => {
                        possible[idx] = (l, limit);
                        total += process_rule(workflows, target, possible);
                        possible[idx] = (limit, u);
                    }
                    ((l, u), b'>') if u >= limit => {
                        possible[idx] = (limit + 1, u);
                        total += process_rule(workflows, target, possible);
                        possible[idx] = (l, limit + 1);
                    }
                    _ => (),
                },
            };
        }
        total
    }

    // Entry point to calculate the total number of accepted combinations for a given input.
    pub fn calculate_accepted_combinations(input: &str) -> i64 {
        let workflows = parse_to_workflows(input);
        process_rule(&workflows, "in", [(1, 4001); 4])
    }
}
