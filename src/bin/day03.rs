mod helpers;

fn main() {
    let filename: &str = "day03.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    let result = task_1(&input);
    println!("Task 1: {}", result);

    let result = task_2(&input);
    println!("Task 2: {}", result);
}

fn task_1(input: &Vec<String>) -> u32 {
    return count_encountered_trees(input, 3, 1);
}

fn task_2(input: &Vec<String>) -> u32 {
    return count_encountered_trees(input, 1, 1)
        * count_encountered_trees(input, 3, 1)
        * count_encountered_trees(input, 5, 1)
        * count_encountered_trees(input, 7, 1)
        * count_encountered_trees(input, 1, 2);
}

fn count_encountered_trees(input: &Vec<String>, r_step: usize, d_step: usize) -> u32 {
    let mut encountered_tree_count = 0;
    let mut x_idx: usize = 0;
    let mut y_idx: usize = 0;

    while y_idx < input.len() {
        if input[y_idx].chars().nth(x_idx).unwrap() == '#' {
            encountered_tree_count += 1;
        }
        x_idx = (x_idx + r_step) % input[y_idx].len();
        y_idx += d_step;
    }

    return encountered_tree_count;
}

#[cfg(test)]
mod tests {

    fn get_example_case() -> Vec<String> {
        return vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
    }

    #[test]
    fn verify_example_task_1() {
        let result = crate::task_1(&get_example_case());

        assert_eq!(7, result);
    }

    #[test]
    fn verify_example_task_2() {
        let result = crate::task_2(&get_example_case());

        assert_eq!(336, result);
    }
}
