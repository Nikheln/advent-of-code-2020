mod helpers;

fn main() {
    let filename: &str = "day01.txt";
    let input = helpers::input_helpers::read_input(&filename)
        .unwrap()
        .iter()
        .map(|r| r.parse::<u32>().unwrap())
        .collect();

    let result = find_multiple_of_pair_with_sum(&input, 2020);

    match result {
        Some(answer) => println!("Task 1: {}", answer),
        None => println!("Task 1: No product found!"),
    }

    let result = find_multiple_of_triplet_with_sum(&input, 2020);

    match result {
        Some(answer) => println!("Task 2: {}", answer),
        None => println!("Task 2: No product found!"),
    }
}

fn find_multiple_of_pair_with_sum(input: &Vec<u32>, wanted_sum: u32) -> Option<u32> {
    for val1 in input {
        for val2 in input {
            if val1 + val2 == wanted_sum {
                return Some(val1 * val2);
            }
        }
    }

    None
}

fn find_multiple_of_triplet_with_sum(input: &Vec<u32>, wanted_sum: u32) -> Option<u32> {
    for val1 in input {
        for val2 in input {
            for val3 in input {
                if val1 + val2 +val3 == wanted_sum {
                    return Some(val1 * val2 * val3);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::find_multiple_of_pair_with_sum;
    use crate::find_multiple_of_triplet_with_sum;

    #[test]
    fn verify_example_task_1() {
        let result = find_multiple_of_pair_with_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        match result {
            Some(product) => assert_eq!(514579, product),
            None => panic!(),
        }
    }

    #[test]
    fn verify_example_task_2() {
        let result = find_multiple_of_triplet_with_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020);
        match result {
            Some(product) => assert_eq!(241861950, product),
            None => panic!(),
        }
    }
}
