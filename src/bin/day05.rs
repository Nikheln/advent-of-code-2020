use std::collections::BTreeSet;
mod helpers;

fn main() {
    let filename: &str = "day05.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) {
    let max_seat_id = input.iter().map(|r| parse_seat_id(r)).max();

    println!("Task 1: {}", max_seat_id.unwrap());
}

fn task_2(input: &[String]) {
    let seat_ids = input.iter().map(|r| parse_seat_id(r));
    let testing_range = seat_ids.clone().min().unwrap()..=seat_ids.clone().max().unwrap();
    let seat_id_set: BTreeSet<u16> = seat_ids.collect();

    for potential_seat_id in testing_range {
        if !seat_id_set.contains(&potential_seat_id)
            && seat_id_set.contains(&(potential_seat_id - 1))
            && seat_id_set.contains(&(potential_seat_id + 1))
        {
            println!("Task 2: {}", potential_seat_id);
            return;
        }
    }
    println!("Task 2: -");
}

fn parse_seat_id(seat_def: &str) -> u16 {
    // Construct a number from the whole definition, and extract row and column with bitwise operations
    let numeric_def = u16::from_str_radix(
        &seat_def
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    );

    numeric_def.unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_seat_id() {
        assert_eq!(567, crate::parse_seat_id(&"BFFFBBFRRR".to_string()));
        assert_eq!(119, crate::parse_seat_id(&"FFFBBBFRRR".to_string()));
        assert_eq!(820, crate::parse_seat_id(&"BBFFBBFRLL".to_string()));
    }
}
