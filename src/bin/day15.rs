use std::collections::HashMap;

mod helpers;

fn main() {
    let filename: &str = "day15.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap()[0];
    task_1(input);
    task_2(input);
}

fn task_1(input: &String) -> u64 {
    let result = get_nth_in_serise(input, 2020);
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &String) -> u64 {
    let result = get_nth_in_serise(input, 30000000);
    println!("Task 2: {}", result);
    result
}

fn get_nth_in_serise(input: &String, n: usize) -> u64 {
    let input_values: Vec<(u64, usize)> = input
        .split_terminator(',')
        .enumerate()
        .map(|(idx, val)| (val.parse::<u64>().unwrap(), idx))
        .collect();
    let (last, rest) = input_values.split_last().unwrap();
    let (mut latest_value, mut latest_idx) = last;
    let mut last_occurrences: HashMap<u64, usize> = rest.to_owned().into_iter().collect();

    while latest_idx < n - 1 {
        let last_occurrence = last_occurrences.get(&latest_value);
        let next_value = match last_occurrence {
            Some(last_instance_idx) => (latest_idx - last_instance_idx) as u64,
            None                    => 0
        };
        last_occurrences.insert(latest_value, latest_idx);
        latest_idx += 1;
        latest_value = next_value;
    }

    latest_value
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        assert_eq!(436, crate::task_1(&"0,3,6".to_string()));
        assert_eq!(1, crate::task_1(&"1,3,2".to_string()));
        assert_eq!(10, crate::task_1(&"2,1,3".to_string()));
        assert_eq!(27, crate::task_1(&"1,2,3".to_string()));
        assert_eq!(78, crate::task_1(&"2,3,1".to_string()));
        assert_eq!(438, crate::task_1(&"3,2,1".to_string()));
        assert_eq!(1836, crate::task_1(&"3,1,2".to_string()));
    }

    #[test]
    fn verify_example_task_2() {
        assert_eq!(175594, crate::task_2(&"0,3,6".to_string()));
        assert_eq!(2578, crate::task_2(&"1,3,2".to_string()));
        assert_eq!(3544142, crate::task_2(&"2,1,3".to_string()));
        assert_eq!(261214, crate::task_2(&"1,2,3".to_string()));
        assert_eq!(6895259, crate::task_2(&"2,3,1".to_string()));
        assert_eq!(18, crate::task_2(&"3,2,1".to_string()));
        assert_eq!(362, crate::task_2(&"3,1,2".to_string()));
    }
}
