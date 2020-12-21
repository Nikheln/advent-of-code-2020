use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

mod helpers;

fn main() {
    let filename: &str = "day16.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u32 {
    let (rules, _, other_tickets) = parse_input(input);
    let mut result = 0;
    for ticket in other_tickets {
        for field in ticket.field_values {
            let field_valid = rules.iter().any(|r| r.valid_values.contains(&field));
            if !field_valid {
                result += field;
            }
        }
    }
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u64 {
    let (rules, own_ticket, other_tickets) = parse_input(input);
    let valid_tickets: Vec<&Ticket> = other_tickets
        .iter()
        .filter(|t| {
            for field in &t.field_values {
                if !rules.iter().any(|r| r.valid_values.contains(&field)) {
                    return false;
                }
            }
            true
        })
        .collect();

    let rule_potential_indices: Vec<(&FieldRule, Vec<usize>)> = rules
        .iter()
        .map(|r| (r, r.potential_indices(&valid_tickets)))
        .collect();
    // Assume there is always at least one rule that has only one valid index
    let mut rule_locked_indices: HashMap<&FieldRule, usize> = HashMap::new();
    while rule_locked_indices.len() < rules.len() {
        for (rule, potential_indices) in &rule_potential_indices {
            if rule_locked_indices.contains_key(rule) {
                continue;
            }
            let available_indices: Vec<&usize> = potential_indices
                .iter()
                .filter(|idx| !rule_locked_indices.values().any(|v| &v == idx))
                .collect();
            if available_indices.len() == 1 {
                rule_locked_indices.insert(&rule, *available_indices[0]);
                continue;
            }
        }
    }

    let result: u64 = rule_locked_indices
        .iter()
        .filter(|(r, _)| r.name.starts_with("departure"))
        .map(|(_, idx)| own_ticket.field_values[*idx] as u64)
        .product();

    println!("Task 2: {}", result);
    result
}

fn parse_input(input: &[String]) -> (Vec<FieldRule>, Ticket, Vec<Ticket>) {
    let mut idx: usize = 0;

    let mut rules: Vec<FieldRule> = Vec::new();
    while !input[idx].is_empty() {
        rules.push(input[idx].parse::<FieldRule>().unwrap());
        idx += 1;
    }
    idx += 2;
    let own_ticket = input[idx].parse::<Ticket>().unwrap();

    idx += 3;
    let other_tickets: Vec<Ticket> = input[idx..]
        .iter()
        .map(|v| v.parse::<Ticket>().unwrap())
        .collect();
    (rules, own_ticket, other_tickets)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct FieldRule {
    name: String,
    valid_values: Vec<u32>,
}

impl FieldRule {
    fn potential_indices(&self, tickets: &[&Ticket]) -> Vec<usize> {
        (0..tickets[0].field_values.len())
            .into_iter()
            .filter(|i| {
                tickets
                    .iter()
                    .all(|t| self.valid_values.contains(&t.field_values[*i]))
            })
            .collect()
    }
}

impl FromStr for FieldRule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_terminator(": ").collect();
        let valid_vals: Vec<u32> = parts[1]
            .split_terminator(" or ")
            .map(|interval| {
                let interval_ends: Vec<&str> = interval.split_terminator('-').collect();
                ((interval_ends[0].parse::<u32>().unwrap())
                    ..=(interval_ends[1].parse::<u32>().unwrap()))
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();
        Ok(FieldRule {
            name: parts[0].to_owned(),
            valid_values: valid_vals,
        })
    }
}

struct Ticket {
    field_values: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            field_values: s
                .split_terminator(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        assert_eq!(71, crate::task_1(&input));
    }
}
