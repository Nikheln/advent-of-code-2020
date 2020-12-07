use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

mod helpers;

lazy_static! {
    static ref BAG_DEF_RE: Regex = Regex::new(r"^(.*?) bags contain (.*)$").unwrap();
    static ref INCLUDED_BAG_RE: Regex = Regex::new(r"\s*(\d+)\s+(.*?)\s+bags?.?").unwrap();
}

fn main() {
    let filename: &str = "day07.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> usize {
    let bag_graph = parse_bag_graph(input);

    let mut including_bags: HashSet<&String> = HashSet::new();
    let mut stack: Vec<&str> = vec!["shiny gold"];

    while let Some(top) = stack.pop() {
        for bag in bag_graph.keys().filter(|k| k.1 == top) {
            if !including_bags.contains(&bag.0) {
                stack.push(&bag.0);
                including_bags.insert(&bag.0);
            }
        }
    }

    println!("Task 1: {}", including_bags.len());

    including_bags.len()
}

fn task_2(input: &[String]) -> u32 {
    let bag_graph = parse_bag_graph(input);

    // Subtract the bag itself
    let nested_bags = count_nesting_bags(&bag_graph, "shiny gold") - 1;

    println!("Task 2: {}", nested_bags);

    nested_bags
}

fn count_nesting_bags(bag_graph: &HashMap<(String, String), u8>, bag_color: &str) -> u32 {
    let mut total: u32 = 1;

    for nested_bag_key in bag_graph.keys().filter(|k| k.0 == bag_color) {
        total += *bag_graph.get(nested_bag_key).unwrap() as u32
            * count_nesting_bags(bag_graph, &nested_bag_key.1);
    }

    total
}

fn parse_bag_graph(input: &[String]) -> HashMap<(String, String), u8> {
    // The tuple contains the (source, target) colors, and the value stores the amount of
    // (target) bags stored in one (source) bag
    let mut bag_relations: HashMap<(String, String), u8> = HashMap::new();

    for bag_def in input {
        if let Some(bag_captures) = BAG_DEF_RE.captures(bag_def) {
            let bag_color = &bag_captures[1];
            let inc_bags = &bag_captures[2];

            for inc_bag_def in inc_bags.split(',') {
                if let Some(inc_bag_captures) = INCLUDED_BAG_RE.captures(inc_bag_def) {
                    let inc_bag_color = inc_bag_captures[2].trim().to_string();
                    let inc_bag_count = inc_bag_captures[1].parse::<u8>().unwrap();

                    bag_relations
                        .insert((bag_color.trim().to_string(), inc_bag_color), inc_bag_count);
                }
            }
        }
    }

    bag_relations
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        assert_eq!(4, crate::task_1(&input));
    }

    #[test]
    fn verify_example_task_2_sample_1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        assert_eq!(32, crate::task_2(&input));
    }

    #[test]
    fn verify_example_task_2_sample_2() {
        let input = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];
        assert_eq!(126, crate::task_2(&input));
    }
}
