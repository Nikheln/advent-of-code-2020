use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

mod helpers;

lazy_static! {
    static ref MASK_UPDATE_RE: Regex = Regex::new(r"^mask = ([X01]+)$").unwrap();
    static ref MEM_WRITE_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)$").unwrap();
}

fn main() {
    let filename: &str = "day14.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();
    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u64 {
    // The input data seems to only access memory slots under 2^16
    const MEM_SIZE: usize = 1 << 16;
    // The values are 36 bits long, so u32 is insufficient
    // All values are initialized as 0
    let mut memory: [u64; MEM_SIZE] = [0; MEM_SIZE];
    let mut mask: Bitmask = "X".repeat(36).parse::<Bitmask>().unwrap();

    for elem in input {
        if let Some(captures) = MASK_UPDATE_RE.captures(&elem) {
            mask = captures[1].parse::<Bitmask>().unwrap();
        } else if let Some(captures) = MEM_WRITE_RE.captures(&elem) {
            let mem_idx = captures[1].parse::<usize>().unwrap();
            let mut value = captures[2].parse::<u64>().unwrap();

            value = mask.apply(value);

            memory[mem_idx] = value;
        }
    }

    let result = memory.iter().sum();
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: Bitmask = "X".repeat(36).parse::<Bitmask>().unwrap();
    for elem in input {
        if let Some(captures) = MASK_UPDATE_RE.captures(&elem) {
            mask = captures[1].parse::<Bitmask>().unwrap();
        } else if let Some(captures) = MEM_WRITE_RE.captures(&elem) {
            let mem_idx = captures[1].parse::<u64>().unwrap();
            let value = captures[2].parse::<u64>().unwrap();

            // Brute-force through all memory indices, check if the index maches,
            // excluding the floating indices
            for idx in get_indices(mem_idx, &mask) {
                memory.insert(idx, value);
            }
        }
    }

    let result = memory.values().sum();
    println!("Task 2: {}", result);
    result
}

fn get_indices(index: u64, mask: &Bitmask) -> Vec<u64> {
    let floating_indices = &mask.floating_indices;
    let input_str = to_bitstring(index | mask.ones);

    // Initialize the first substring up to the first floating bit
    let baseline = input_str[..floating_indices[0]].to_owned();
    let mut work: Vec<String> = vec![baseline];

    // On every step, add two versions of all existing strings (one suffixed with 0, one
    // with 1) and prepend the strings until the index
    for i in 0..floating_indices.len() {
        let this_floater: usize = floating_indices[i]+1;
        let next_floater: usize = match i {
            val if val < floating_indices.len() - 1 => floating_indices[val+1],
            _              => input_str.len(),
        };
        let substr = &input_str[this_floater..next_floater];

        let mut new_entries: Vec<String> = Vec::new();
        new_entries.extend(
            work.iter()
                .map(|x| [x, "0", substr].concat()),
        );
        new_entries.extend(
            work.iter()
                .map(|x| [x, "1", substr].concat()),
        );

        work.clear();
        work.append(&mut new_entries);
    }

    work.iter().map(|x| u64::from_str_radix(x, 2).unwrap()).collect()
}

fn to_bitstring(input: u64) -> String {
    format!("{:0>36}", format!("{:b}", input))
}

struct Bitmask {
    zeroes: u64,
    ones: u64,
    floating_indices: Vec<usize>
}

impl Bitmask {
    fn apply(&self, input: u64) -> u64 {
        input & self.zeroes | self.ones
    }
}

impl FromStr for Bitmask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bitmask {
            zeroes: u64::from_str_radix(&s.replace("X", "1"), 2).unwrap(),
            ones: u64::from_str_radix(&s.replace("X", "0"), 2).unwrap(),
            floating_indices: s.char_indices().filter(|c| c.1 == 'X').map(|c| c.0).collect()
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        assert_eq!(165, crate::task_1(&input));
    }

    #[test]
    fn verify_example_task_2() {
        let input = vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];
        assert_eq!(208, crate::task_2(&input));
    }

    #[test]
    fn verify_get_indices() {
        assert_eq!(vec![1, 1+(2 as u64).pow(34)], crate::get_indices(1, &"0X0000000000000000000000000000000000".parse::<crate::Bitmask>().unwrap()));
        assert_eq!(vec![1, 1+(2 as u64).pow(35)], crate::get_indices(1, &"X00000000000000000000000000000000000".parse::<crate::Bitmask>().unwrap()));
    }
}
