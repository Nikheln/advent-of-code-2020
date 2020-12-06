use std::collections::BTreeSet;
mod helpers;

fn main() {
    let filename: &str = "day06.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u32 {
    let mut total_count: u32 = 0;

    let mut current_group_answers: BTreeSet<char> = BTreeSet::new();

    for row in input {
        if row.is_empty() {
            total_count += current_group_answers.len() as u32;
            current_group_answers.clear();
        } else {
            current_group_answers.extend(row.chars());
        }
    }
    total_count += current_group_answers.len() as u32;

    println!("Task 1: {}", total_count);
    total_count
}

fn task_2(input: &[String]) -> u32{
    let mut total_count: u32 = 0;

    let mut current_group_answers: BTreeSet<char> = BTreeSet::new();
    let mut current_row_values: BTreeSet<char> = BTreeSet::new();
    let mut is_new_group = true;

    for row in input {
        current_row_values.clear();
        current_row_values.extend(row.chars());

        if current_row_values.is_empty() {
            total_count += current_group_answers.len() as u32;
            current_group_answers.clear();
            is_new_group = true;
        } else if is_new_group {
            current_group_answers = &current_group_answers | &current_row_values;
            is_new_group = false;
        } else {
            // The set of shared answers is collected by taking an intersection between
            // the group's answers and individual passenger's answers, so only those
            // present on all rows are left once the group has been iterated
            current_group_answers = &current_group_answers & &current_row_values;
        }
    }
    total_count += current_group_answers.len() as u32;

    println!("Task 2: {}", total_count);
    total_count
}


#[cfg(test)]
mod tests {

    fn get_example_case() -> Vec<String> {
        vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ]
    }
    #[test]
    fn verify_example_task_1() {
        assert_eq!(11, crate::task_1(&get_example_case()));
    }

    #[test]
    fn verify_example_task_2() {

        assert_eq!(6, crate::task_2(&get_example_case()));
    }
}
