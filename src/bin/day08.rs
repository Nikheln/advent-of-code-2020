mod helpers;

fn main() {
    let filename: &str = "day08.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> i32 {
    let instructions: Vec<Instruction> = input.iter().map(|r| parse_instruction(r)).collect();

    let result = evaluate_program(instructions);

    match result {
        ProgramResult::Killed(arg) => {
            println!("Task 1: {}", arg);
            arg
        }
        ProgramResult::Halted(_) => panic!("Unexpected result!"),
    }
}

fn task_2(input: &[String]) -> i32 {
    let instructions: Vec<Instruction> = input.iter().map(|r| parse_instruction(r)).collect();

    for i in 0..instructions.len() {
        if let Instruction::Acc(_) = instructions[i] {
            // Acc instructions are left untouched, no need to evaluate
            continue;
        }
        // Swap the i'th instruction if it's a Nop or Jmp, and evaluate the program
        let mut instructions_copy: Vec<Instruction> = instructions.to_vec();
        let new_instruction = match instructions[i] {
            Instruction::Acc(arg) => Instruction::Acc(arg),
            Instruction::Jmp(arg) => Instruction::Nop(arg),
            Instruction::Nop(arg) => Instruction::Jmp(arg),
        };
        instructions_copy[i] = new_instruction;

        let result = evaluate_program(instructions_copy);
        match result {
            ProgramResult::Halted(arg) => {
                println!("Task 2: {} by swapping instruction n:o {}", arg, i);
                return arg;
            }
            ProgramResult::Killed(_) => {}
        }
    }
    0
}

fn evaluate_program(instructions: Vec<Instruction>) -> ProgramResult {
    let mut accumulator: i32 = 0;
    let mut current_idx: i32 = 0;
    let mut current_instruction: Option<&Instruction> = instructions.get(current_idx as usize);
    // Stores the indices of already evaluated instructions
    let mut visited_instructions: Vec<i32> = Vec::new();

    while !visited_instructions.contains(&current_idx) && current_idx < instructions.len() as i32 {
        visited_instructions.push(current_idx);
        match current_instruction {
            Some(Instruction::Acc(arg)) => accumulator += arg,
            // Subtract 1 from the resulting index, since the instruction pointer will be
            // incremented after the match
            Some(Instruction::Jmp(arg)) => current_idx = current_idx + arg - 1,
            Some(Instruction::Nop(_)) => {}
            None => {
                break;
            }
        }

        current_idx += 1;
        // Kill the program if the instruction pointer becomes negative
        if current_idx < 0 {
            return ProgramResult::Killed(accumulator);
        }
        current_instruction = instructions.get(current_idx as usize);
    }

    if visited_instructions.contains(&current_idx) {
        ProgramResult::Killed(accumulator)
    } else {
        ProgramResult::Halted(accumulator)
    }
}

fn parse_instruction(row: &str) -> Instruction {
    let parts: Vec<&str> = row.split(' ').collect();
    let arg = parts[1].parse::<i32>().unwrap();

    match parts[0] {
        "acc" => Instruction::Acc(arg),
        "jmp" => Instruction::Jmp(arg),
        "nop" => Instruction::Nop(arg),
        &_ => panic!("Unidentified instruction type"),
    }
}

#[derive(PartialEq, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

/*
 * Represents the output of an executed instruction set. The i32 argument contains the
 * accumulator's value at the time the execution stopped.
 */
enum ProgramResult {
    // The program halted naturally
    Halted(i32),
    // The program was stopped after a loop was detected
    Killed(i32),
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];
        assert_eq!(5, crate::task_1(&input));
    }

    #[test]
    fn verify_example_task_2() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];
        assert_eq!(8, crate::task_2(&input));
    }
}
