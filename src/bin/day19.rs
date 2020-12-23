use std::collections::HashMap;

mod helpers;

fn main() {
    let filename: &str = "day19.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u32 {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut idx = 0;
    while !input[idx].is_empty() {
        let parts: Vec<&str> = input[idx].split(": ").collect();
        rules.insert(parts[0].parse::<usize>().unwrap(), Rule::parse(parts[1]));
        idx += 1;
    }

    let potential_strings: Vec<String> = input[idx + 1..].into();
    let valid_strings = get_valid_strings(rules.get(&0).unwrap(), &rules);

    let result = potential_strings
        .iter()
        .filter(|s| valid_strings.contains(s))
        .count();
    println!("Task 1: {}", result);
    result as u32
}

fn task_2(input: &[String]) -> u32 {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut idx = 0;
    while !input[idx].is_empty() {
        let parts: Vec<&str> = input[idx].split(": ").collect();
        let rule_no = parts[0].parse::<usize>().unwrap();

        if ![0, 8, 11].contains(&rule_no) {
            rules.insert(rule_no, Rule::parse(parts[1]));
        }
        idx += 1;
    }

    let potential_strings: Vec<String> = input[idx + 1..].into();
    // Rule 0 is always "8 11", and task 2's modification caused rules 8 and 11 to
    // recurse so that they always repeat rule 42 some 1..n times, and then rule 31 1..m
    // times, where m < n.
    let valid_prefixes = get_valid_strings(rules.get(&42).unwrap(), &rules);
    let valid_suffixes = get_valid_strings(rules.get(&31).unwrap(), &rules);

    let result = potential_strings
        .iter()
        .filter(|s| validate_task_2(s, &valid_prefixes, &valid_suffixes, 0, 0))
        .count();
    println!("Task 2: {}", result);
    result as u32
}

fn validate_task_2(
    input: &str,
    prefixes: &[String],
    suffixes: &[String],
    trimmed_prefix_count: u8,
    trimmed_suffix_count: u8,
) -> bool {
    // There need to be more prefixes of rule 42 than suffixes of rule 31, and both need
    // to appear at least once
    if input.len() == 0 {
        return trimmed_prefix_count > trimmed_suffix_count && trimmed_suffix_count > 0;
    }

    let matching_prefixes: Vec<&String> =
        prefixes.iter().filter(|p| input.starts_with(*p)).collect();
    let matching_suffixes: Vec<&String> = suffixes.iter().filter(|s| input.ends_with(*s)).collect();
    for p in matching_prefixes {
        if validate_task_2(
            input.strip_prefix(p).unwrap(),
            prefixes,
            suffixes,
            trimmed_prefix_count + 1,
            trimmed_suffix_count,
        ) {
            return true;
        }
    }
    for s in matching_suffixes {
        if validate_task_2(
            input.strip_suffix(s).unwrap(),
            prefixes,
            suffixes,
            trimmed_prefix_count,
            trimmed_suffix_count + 1,
        ) {
            return true;
        }
    }

    false
}

fn get_valid_strings(seed_rule: &Rule, rulebook: &HashMap<usize, Rule>) -> Vec<String> {
    match seed_rule {
        Rule::Strings(s) => vec![s.to_string()],
        Rule::Alternatives(alt_rules) => alt_rules
            .iter()
            .map(|r| get_valid_strings(r, &rulebook))
            .flatten()
            .collect(),
        Rule::Concatenation(indices) => {
            let mut results: Vec<String> = vec!["".to_string()];
            for idx in indices {
                let this_idx_results = get_valid_strings(&rulebook[idx], &rulebook);
                let mut new_results = Vec::new();
                for rec_result in this_idx_results {
                    for curr_result in &results {
                        new_results.push([curr_result.to_owned(), rec_result.to_owned()].join(""));
                    }
                }
                results = new_results;
            }
            results
        }
    }
}

#[derive(Debug)]
enum Rule {
    Alternatives(Vec<Rule>),
    Concatenation(Vec<usize>),
    Strings(String),
}

impl Rule {
    fn parse(input: &str) -> Rule {
        if input.starts_with('"') {
            Rule::Strings(input.trim_matches('"').into())
        } else if input.contains('|') {
            Rule::Alternatives(input.split('|').map(Rule::parse).collect())
        } else {
            Rule::Concatenation(
                input
                    .split(' ')
                    .map(|v| v.trim().parse::<usize>())
                    .flatten()
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_day19_example_task_1() {
        let input = vec![
            "0: 4 1 5".to_string(),
            "1: 2 3 | 3 2".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "4: \"a\"".to_string(),
            "5: \"b\"".to_string(),
            "".to_string(),
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string(),
        ];
        assert_eq!(2, crate::task_1(&input));
    }

    #[test]
    fn verify_day19_example_task_2() {
        let input = vec![
            "42: 9 14 | 10 1".to_string(),
            "9: 14 27 | 1 26".to_string(),
            "10: 23 14 | 28 1".to_string(),
            "1: \"a\"".to_string(),
            "11: 42 31".to_string(),
            "5: 1 14 | 15 1".to_string(),
            "19: 14 1 | 14 14".to_string(),
            "12: 24 14 | 19 1".to_string(),
            "16: 15 1 | 14 14".to_string(),
            "31: 14 17 | 1 13".to_string(),
            "6: 14 14 | 1 14".to_string(),
            "2: 1 24 | 14 4".to_string(),
            "0: 8 11".to_string(),
            "13: 14 3 | 1 12".to_string(),
            "15: 1 | 14".to_string(),
            "17: 14 2 | 1 7".to_string(),
            "23: 25 1 | 22 14".to_string(),
            "28: 16 1".to_string(),
            "4: 1 1".to_string(),
            "20: 14 14 | 1 15".to_string(),
            "3: 5 14 | 16 1".to_string(),
            "27: 1 6 | 14 18".to_string(),
            "14: \"b\"".to_string(),
            "21: 14 1 | 1 14".to_string(),
            "25: 1 1 | 1 14".to_string(),
            "22: 14 14".to_string(),
            "8: 42".to_string(),
            "26: 14 22 | 1 20".to_string(),
            "18: 15 15".to_string(),
            "7: 14 5 | 1 21".to_string(),
            "24: 14 1".to_string(),
            "".to_string(),
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
            "bbabbbbaabaabba".to_string(),
            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
            "ababaaaaaabaaab".to_string(),
            "ababaaaaabbbaba".to_string(),
            "baabbaaaabbaaaababbaababb".to_string(),
            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
            "aaaaabbaabaaaaababaa".to_string(),
            "aaaabbaaaabbaaa".to_string(),
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
            "babaaabbbaaabaababbaabababaaab".to_string(),
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
        ];

        assert_eq!(12, crate::task_2(&input));
    }
}
