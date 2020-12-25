mod helpers;

fn main() {
    let filename: &str = "day25.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    // Warning: this brute-force approach takes about 20 minutes to complete with complex
    // inputs
    task_1(input);
}

fn task_1(input: &[String]) -> u64 {
    let pubkey_a = input[0].parse::<u64>().unwrap();
    let pubkey_b = input[1].parse::<u64>().unwrap();

    let result: u64;
    let mut privkey = 0;
    'outer: loop {
        match transform(7, privkey) {
            v if v == pubkey_a => {
                result = transform(pubkey_b, privkey);
                break 'outer;
            }
            v if v == pubkey_b => {
                result = transform(pubkey_a, privkey);
                break 'outer;
            }
            _ => {
                privkey += 1;
            }
        }
    }
    println!("Task 1: {}", result);
    result
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    const MODULO: u64 = 20201227;
    let mut v = 1;
    for _ in 0..loop_size {
        v = (v * subject) % MODULO;
    }

    v
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_day25_example_task_1() {
        let input = vec!["17807724".to_string(), "5764801".to_string()];
        assert_eq!(14897079, crate::task_1(&input));
    }
}
