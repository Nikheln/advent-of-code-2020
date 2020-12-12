mod helpers;

fn main() {
    let filename: &str = "day10.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();
    task_1(&input);
    task_2(&input);
}

fn get_all_joltages(input: &[String]) -> Vec<u32> {
    let mut rows: Vec<u32> = input.iter().map(|r| r.parse::<u32>().unwrap()).collect();

    // The input is 0 jolts
    rows.push(0);
    let adapter_jolts = rows.iter().clone().fold(0, |acc, &x| acc.max(x)) + 3;
    // Output is largest adapter joltage + 3
    rows.push(adapter_jolts);

    // Sort the joltages before returning so analysis is easier
    rows.sort();

    rows
}

fn task_1(input: &[String]) -> u32 {
    // Solution idea: calculate differences between consecutive elements, count 1s and 3s
    let rows = get_all_joltages(input);
    let orig = rows.iter();
    let skipped = rows.iter().skip(1);

    let mut counters = vec![0, 0, 0, 0];

    orig.zip(skipped)
        .map(|(&prev, &next)| (next - prev) as usize)
        .for_each(|diff| counters[diff] += 1);

    let result = counters[1] * counters[3];

    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u64 {
    let rows = get_all_joltages(input);

    // Create a memoization array to reduce the recursion's runtime
    let mut memo: Vec<Option<u64>> = vec![None; rows.len()];

    let result = count_paths_rec(&rows, &mut memo, 0);

    println!("Task 2: {}", result);
    result
}

fn count_paths_rec(input: &[u32], memo: &mut [Option<u64>], from_idx: usize) -> u64 {
    if memo[from_idx].is_none() {
        if from_idx == input.len() - 1 {
            memo[from_idx] = Some(1);
        } else {
            let mut subpath_count = 0;
            let this_val = input.get(from_idx).unwrap();
            for i in 1..=3 {
                let next_idx = from_idx + i;
                let next_val = input.get(next_idx);
                if next_val.is_some() && *next_val.unwrap() <= this_val + 3 {
                    subpath_count += count_paths_rec(input, memo, next_idx)
                }
            }
            memo[from_idx] = Some(subpath_count);
        }
    }

    memo[from_idx].unwrap()
}

#[cfg(test)]
mod tests {

    fn get_example_1() -> Vec<String> {
        vec![
            "16".to_string(),
            "10".to_string(),
            "15".to_string(),
            "5".to_string(),
            "1".to_string(),
            "11".to_string(),
            "7".to_string(),
            "19".to_string(),
            "6".to_string(),
            "12".to_string(),
            "4".to_string(),
        ]
    }

    fn get_example_2() -> Vec<String> {
        vec![
            "28".to_string(),
            "33".to_string(),
            "18".to_string(),
            "42".to_string(),
            "31".to_string(),
            "14".to_string(),
            "46".to_string(),
            "20".to_string(),
            "48".to_string(),
            "47".to_string(),
            "24".to_string(),
            "23".to_string(),
            "49".to_string(),
            "45".to_string(),
            "19".to_string(),
            "38".to_string(),
            "39".to_string(),
            "11".to_string(),
            "1".to_string(),
            "32".to_string(),
            "25".to_string(),
            "35".to_string(),
            "8".to_string(),
            "17".to_string(),
            "7".to_string(),
            "9".to_string(),
            "4".to_string(),
            "2".to_string(),
            "34".to_string(),
            "10".to_string(),
            "3".to_string(),
        ]
    }

    #[test]
    fn verify_example_task_1_example_1() {
        assert_eq!(7 * 5, crate::task_1(&get_example_1()));
    }

    #[test]
    fn verify_example_task_1_example_2() {
        assert_eq!(22 * 10, crate::task_1(&get_example_2()));
    }

    #[test]
    fn verify_example_task_2_example_1() {
        assert_eq!(8, crate::task_2(&get_example_1()));
    }

    #[test]
    fn verify_example_task_2_example_2() {
        assert_eq!(19208, crate::task_2(&get_example_2()));
    }
}
