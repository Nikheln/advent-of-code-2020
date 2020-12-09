use std::collections::VecDeque;

mod helpers;

fn main() {
    let filename: &str = "day09.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) {
    println!("Task 1: {}", task_1_internal(input, 25));
}

fn task_1_internal(input: &[String], preamble_size: usize) -> i64 {
    let rows: Vec<i64> = input.iter().map(|r| r.parse::<i64>().unwrap()).collect();

    // Collect the possible value to a queue so that the first items are the input's first
    // value added with the other preamble values, then the second value added with the
    // other preamble values, etc.
    // This way the "oldest" valid values can be dropped and new ones can be inserted on
    // every item
    let mut valid_values: VecDeque<i64> = VecDeque::new();
    // Populate the collection of valid values with the initial preamble
    for i in 0..preamble_size {
        for j in 0..preamble_size {
            if i != j {
                valid_values.push_front(rows[i] + rows[j]);
            }
        }
    }

    for i in preamble_size..rows.len() {
        let ith_value = rows.get(i).unwrap();
        if !valid_values.contains(ith_value) {
            return *ith_value;
        }

        for j in (i - preamble_size + 1)..=i {
            valid_values.pop_back();
            valid_values.push_front(rows[i] + rows[j]);
        }
    }

    0
}

fn task_2(input: &[String]) {
    println!("Task 2: {}", task_2_internal(input, 25));
}

fn task_2_internal(input: &[String], preamble_size: usize) -> i64 {
    let rows: Vec<i64> = input.iter().map(|r| r.parse::<i64>().unwrap()).collect();
    let invalid_number = task_1_internal(input, preamble_size);

    let mut range_start_incl: usize = 0;
    let mut range_end_incl: usize = 0;
    let mut current_sum: i64 = rows[0];

    while range_end_incl < rows.len() {
        if current_sum < invalid_number {
            range_end_incl += 1;
            current_sum += rows[range_end_incl];
        } else if current_sum > invalid_number {
            current_sum -= rows[range_start_incl];
            range_start_incl += 1;
        } else {
            let slice = &rows[range_start_incl..=range_end_incl];
            let result = slice.iter().min().unwrap() + slice.iter().max().unwrap();

            return result;
        }
    }

    panic!("No suitable range found!");
}

#[cfg(test)]
mod tests {

    fn get_example_case() -> Vec<String> {
        vec![
            "35".to_string(),
            "20".to_string(),
            "15".to_string(),
            "25".to_string(),
            "47".to_string(),
            "40".to_string(),
            "62".to_string(),
            "55".to_string(),
            "65".to_string(),
            "95".to_string(),
            "102".to_string(),
            "117".to_string(),
            "150".to_string(),
            "182".to_string(),
            "127".to_string(),
            "219".to_string(),
            "299".to_string(),
            "277".to_string(),
            "309".to_string(),
            "576".to_string(),
        ]
    }

    #[test]
    fn verify_example_task_1() {
        assert_eq!(127, crate::task_1_internal(&get_example_case(), 5));
    }

    #[test]
    fn verify_example_task_2() {
        assert_eq!(62, crate::task_2_internal(&get_example_case(), 5));
    }
}
