use std::collections::VecDeque;
mod helpers;

fn main() {
    let filename: &str = "day23.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap()[0];
    task_1(input);
    task_2(input);
}

fn task_1(input: &String) -> String {
    let mut cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<VecDeque<u64>>();

    for _ in 0..100 {
        play_round(&mut cups);
    }

    let final_state = cups
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");
    let result = final_state.split('1').rev().collect();
    println!("Task 1: {}", result);
    result
}

fn play_round(cups: &mut VecDeque<u64>) {
    let cup_count = cups.len() as u64;

    let current_value = cups.pop_front().unwrap();
    let shifted_values = vec![
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    ];

    let mut target_value = current_value;

    while target_value == current_value || shifted_values.contains(&target_value) {
        if target_value == 1 {
            target_value = cup_count;
        } else {
            target_value -= 1;
        }
    }

    let target_idx = cups.iter().position(|v| v == &target_value).unwrap();

    for s in shifted_values.iter().rev() {
        cups.insert(target_idx + 1, *s);
    }
    // Shift the first item to last
    cups.push_back(current_value);
}

fn task_2(input: &String) -> u64 {
    const CUP_COUNT: usize = 1_000_000;
    // Create an array with 1 000 001 elements, indices ranging from 0 to 1 000 000.
    // Since modeling a linked list is quite difficult in Rust, simulate a linked list's
    // behaviour with this array. The array indices are list's elements (running from 1
    // to 1 000 000), and the values are next elements' values. The list length is longer
    // by one so the indices match, element 0 will not be used.
    let mut cups = vec![0; CUP_COUNT+1];
    
    let init_cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain((input.len() + 1)..=CUP_COUNT)
        .collect::<Vec<usize>>();
    for i in 0..init_cups.len() {
        cups[init_cups[i]] = init_cups[(i+1) % init_cups.len()];
    }

    let mut current = init_cups[0];
    for _ in 0..10_000_000 {
        let first = cups[current];
        let second = cups[first];
        let third = cups[second];

        // Change current value to point to the one after the three removed values
        cups[current] = cups[third];

        let mut target_value = current;
        while [current, first, second, third].contains(&target_value) {
            if target_value == 1{
                target_value = CUP_COUNT;
            } else {
                target_value -= 1;
            }
        }
        let target_next = cups[target_value];
        cups[target_value] = first;
        cups[third] = target_next;

        current = cups[current];
    }
    let result = (cups[1] as u64) * (cups[cups[1]] as u64);
    println!("Task 2: {}", result);
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_day23_example_task_1() {
        let input = "389125467".to_string();
        assert_eq!("67384529", crate::task_1(&input));
    }

    #[test]
    fn verify_day23_example_task_2() {
        let input = "389125467".to_string();
        assert_eq!(149245887792, crate::task_2(&input));
    }
}
