extern crate regex;
mod helpers;

fn main() {
    let filename: &str = "day02.txt";
    let input = helpers::input_helpers::read_input(&filename)
        .unwrap()
        .join(r"\n");

    let result = count_valid_passwords_task_1(&input);

    println!("Task 1: {}", result);
}

fn count_valid_passwords_task_1(input: &str) -> u32 {
    let mut valid_password_count = 0;
    let re = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    for cap in re.captures_iter(input) {
        let min_count = &cap[1].parse::<usize>().unwrap();
        let max_count = &cap[2].parse::<usize>().unwrap();
        let character = &cap[3];
        let password = &cap[4];

        let match_count = password.matches(character).count();
        if min_count <= &match_count && &match_count <= max_count {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}

fn count_valid_passwords_task_2(input: &str) -> u32 {
    let mut valid_password_count = 0;
    let re = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    for cap in re.captures_iter(input) {
        let position_1 = cap[1].parse::<usize>().unwrap() - 1;
        let position_2 = cap[2].parse::<usize>().unwrap() - 1;
        let character = cap[3].chars().next().unwrap();
        let password = &cap[4];

        if (password.chars().nth(position_1).unwrap() == character)
            != (password.chars().nth(position_2).unwrap() == character)
        {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"
            .to_string();
        let result = crate::count_valid_passwords_task_1(&input);

        assert_eq!(2, result);
    }

    #[test]
    fn verify_example_task_2() {
        let input = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"
            .to_string();
        let result = crate::count_valid_passwords_task_2(&input);

        assert_eq!(1, result);
    }
}
