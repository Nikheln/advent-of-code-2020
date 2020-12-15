mod helpers;

fn main() {
    let filename: &str = "day13.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();
    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u32 {
    let offset = input[0].parse::<u32>().unwrap();
    let schedules: Vec<&str> = input[1].split_terminator(',').collect();

    let valid_schedules = schedules
        .iter()
        .filter(|s| s != &&"x")
        .map(|s| s.parse::<u32>().unwrap());
    let next_shift: u32 = valid_schedules
        .min_by(|x, y| {
            get_smallest_multiple_over(*x, offset).cmp(&(get_smallest_multiple_over(*y, offset)))
        })
        .unwrap();
    let wait_time = ((offset / next_shift) + 1) * next_shift - offset;

    let result = next_shift * wait_time;
    println!("Task 1: {:?}", result);
    result
}

fn task_2(input: &[String]) -> i64 {
    let offset = &input[0];
    // Tuples of array index and timing
    let schedules: Vec<(i64, i64)> = input[1]
        .split_terminator(',')
        .map(|x| x.parse::<i64>().ok())
        .enumerate()
        .filter(|(_, res)| res.is_some())
        .map(|(idx, res)| (idx as i64, res.unwrap()))
        .collect();
    
    let mut t = match offset.len() {
        0 => 0,
        _ => 100000000000000
    };

    let mut inc = 1;
    for (idx, bus) in &schedules {
        while (t + idx) % bus != 0 {
            t += inc;
        }

        inc *= bus;
    }

    for (idx, bus) in schedules {
        println!("Asserting {} + {} == 0 mod {}", t, idx, bus);
        assert_eq!(0, (t+idx) % bus);
    }
    
    let result = t;
    println!("Task 2: {}", result);
    result
}

fn get_smallest_multiple_over(val: u32, threshold: u32) -> u32 {
    ((threshold as f64 / val as f64).ceil() * val as f64) as u32
}

#[cfg(test)]
mod tests {

    fn get_example() -> Vec<String> {
        vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()]
    }

    #[test]
    fn verify_example_task_1() {
        assert_eq!(295, crate::task_1(&get_example()));
    }

    #[test]
    fn verify_example_task_2() {
        assert_eq!(
            1068781,
            crate::task_2(&vec![String::new(), "7,13,x,x,59,x,31,19".to_string()])
        );
        assert_eq!(
            3417,
            crate::task_2(&vec![String::new(), "17,x,13,19".to_string()])
        );
        assert_eq!(
            754018,
            crate::task_2(&vec![String::new(), "67,7,59,61".to_string()])
        );
        assert_eq!(
            779210,
            crate::task_2(&vec![String::new(), "67,x,7,59,61".to_string()])
        );
        assert_eq!(
            1261476,
            crate::task_2(&vec![String::new(), "67,7,x,59,61".to_string()])
        );
        assert_eq!(
            1202161486,
            crate::task_2(&vec![String::new(), "1789,37,47,1889".to_string()])
        );
    }
}
