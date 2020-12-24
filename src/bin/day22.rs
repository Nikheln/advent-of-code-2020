use std::collections::VecDeque;
mod helpers;

fn main() {
    let filename: &str = "day22.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u32 {
    let (mut p1_stack, mut p2_stack) = parse_stacks(input);
    while p1_stack.is_empty() == p2_stack.is_empty() {
        let p1_card = p1_stack.pop_front().unwrap();
        let p2_card = p2_stack.pop_front().unwrap();

        if p1_card > p2_card {
            p1_stack.push_back(p1_card);
            p1_stack.push_back(p2_card);
        } else {
            p2_stack.push_back(p2_card);
            p2_stack.push_back(p1_card);
        }
    }
    let winning_stack = match p1_stack.is_empty() {
        true => p2_stack,
        false => p1_stack,
    };

    let result = count_score(&winning_stack);
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u32 {
    let (mut p1_stack, mut p2_stack) = parse_stacks(input);

    let (_, result) = play_recursive_combat(&mut p1_stack, &mut p2_stack);
    println!("Task 2: {}", result);
    result
}

fn play_recursive_combat(p1_stack: &mut VecDeque<u32>, p2_stack: &mut VecDeque<u32>) -> (u8, u32) {
    let mut round_history: Vec<String> = Vec::new();

    while p1_stack.is_empty() == p2_stack.is_empty() {
        // If there was a previous round in this game that had exactly the same cards in
        // the same order in the same players' decks, the game instantly ends in a win for
        // player 1.
        let round_desc = serialize_round(&p1_stack, &p2_stack);
        if round_history.contains(&round_desc) {
            return (1, count_score(&p1_stack));
        }
        round_history.push(round_desc);

        let p1_card = p1_stack.pop_front().unwrap();
        let p1_card_u = p1_card as usize;
        let p2_card = p2_stack.pop_front().unwrap();
        let p2_card_u = p2_card as usize;

        let p1_wins_round;

        if p1_stack.len() >= p1_card_u && p2_stack.len() >= p2_card_u {
            // If both players have at least as many cards remaining in their deck as the
            // value of the card they just drew, the winner of the round is determined by
            // playing a new game of Recursive Combat
            let (winner, _) = play_recursive_combat(
                &mut p1_stack.iter().cloned().take(p1_card_u).collect(),
                &mut p2_stack.iter().cloned().take(p2_card_u).collect(),
            );

            p1_wins_round = winner == 1;
        } else {
            // Otherwise, at least one player must not have enough cards left in their
            // deck to recurse; the winner of the round is the player with the
            // higher-value card.
            p1_wins_round = p1_card > p2_card;
        }

        if p1_wins_round {
            p1_stack.push_back(p1_card);
            p1_stack.push_back(p2_card);
        } else {
            p2_stack.push_back(p2_card);
            p2_stack.push_back(p1_card);
        }
    }

    if p1_stack.is_empty() {
        (2, count_score(&p2_stack))
    } else {
        (1, count_score(&p1_stack))
    }
}

fn serialize_round(p1_stack: &VecDeque<u32>, p2_stack: &VecDeque<u32>) -> String {
    [
        "P1".to_string(),
        p1_stack
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("#"),
        "P2".to_string(),
        p2_stack
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("#"),
    ]
    .join("//")
}

fn count_score(stack: &VecDeque<u32>) -> u32 {
    // The topmost (i.e. first) card gets multiplied by the stack size, and the bottommost
    // one is multiplied by 1, so the indices need to be subtracted from the stack's size
    // to get correct multipliers
    let stack_size = stack.len();
    stack
        .iter()
        .enumerate()
        .map(|(idx, value)| (stack_size - idx) as u32 * value)
        .sum()
}

fn parse_stacks(input: &[String]) -> (VecDeque<u32>, VecDeque<u32>) {
    let separator_row_idx = input.iter().position(|x| x.is_empty()).unwrap();
    let p1_stack = input[1..separator_row_idx]
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    let p2_stack = input[separator_row_idx + 2..]
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    (p1_stack, p2_stack)
}

#[cfg(test)]
mod tests {

    fn get_example() -> Vec<String> {
        vec![
            "Player 1:".to_string(),
            "9".to_string(),
            "2".to_string(),
            "6".to_string(),
            "3".to_string(),
            "1".to_string(),
            "".to_string(),
            "Player 2:".to_string(),
            "5".to_string(),
            "8".to_string(),
            "4".to_string(),
            "7".to_string(),
            "10".to_string(),
        ]
    }
    #[test]
    fn verify_day22_example_task_1() {
        let input = get_example();
        assert_eq!(306, crate::task_1(&input));
    }

    #[test]
    fn verify_day22_example_task_2() {
        let input = get_example();
        assert_eq!(291, crate::task_2(&input));
    }
}
