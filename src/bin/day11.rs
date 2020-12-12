use std::ops::Range;
use std::fmt;

mod helpers;

fn main() {
    let filename: &str = "day11.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();
    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u32 {
    let result = get_occupied_seats_in_end_state(input, &get_next_cell_state_v1);

    if let Some(seats) = result {
        println!("Task 1: {}", seats);
        seats
    } else {
        panic!("Task 1: Failed");
    }
}

fn task_2(input: &[String]) -> u32 {
    let result = get_occupied_seats_in_end_state(input, &get_next_cell_state_v2);

    if let Some(seats) = result {
        println!("Task 1: {}", seats);
        seats
    } else {
        panic!("Task 1: Failed");
    }
}

fn get_occupied_seats_in_end_state(
    input: &[String],
    evaluator: &dyn Fn(&State, i32, i32) -> CellState,
) -> Option<u32> {
    let mut state = parse_init_state(input);

    let max_iteration_count = 1000;
    for _round in 1..=max_iteration_count {
        let prev_state = state;
        state = timestep(&prev_state, evaluator);
        //println!("{}", state);
        if prev_state == state {
            return Some(state.get_state_count(CellState::Occupied));
        }
    }
    None
}

fn timestep(
    prev_state: &State,
    evaluator: &dyn Fn(&State, i32, i32) -> CellState,
) -> State {
    State::new(prev_state.row_range()
        .map(|r| {
            prev_state.col_range()
                .map(|c| evaluator(&prev_state, r, c))
                .collect()
        })
        .collect())
}

fn get_next_cell_state_v1(
    state: &State,
    row_idx: i32,
    col_idx: i32,
) -> CellState {
    let mut surrounding_occupied: u16 = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(CellState::Occupied) = get_visible_cell_in_direction(state, row_idx, col_idx, i, j, false) {
                surrounding_occupied += 1;
            }
        }
    }
    match &state.get_cell(row_idx, col_idx) {
        Some(CellState::Empty) if surrounding_occupied == 0 => CellState::Occupied,
        Some(CellState::Empty) => CellState::Empty,
        Some(CellState::Occupied) if surrounding_occupied >= 4 => CellState::Empty,
        Some(CellState::Occupied) => CellState::Occupied,
        Some(CellState::Floor) => CellState::Floor,
        None => panic!("Invalid cell in coordinates <{},{}>", row_idx, col_idx)
    }
}

fn get_next_cell_state_v2(
    state: &State,
    row_idx: i32,
    col_idx: i32,
) -> CellState {
    let mut surrounding_occupied: u16 = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(CellState::Occupied) = get_visible_cell_in_direction(state, row_idx, col_idx, i, j, true) {
                surrounding_occupied += 1;
            }
        }
    }
    match &state.get_cell(row_idx, col_idx) {
        Some(CellState::Empty) if surrounding_occupied == 0 => CellState::Occupied,
        Some(CellState::Empty) => CellState::Empty,
        Some(CellState::Occupied) if surrounding_occupied >= 5 => CellState::Empty,
        Some(CellState::Occupied) => CellState::Occupied,
        Some(CellState::Floor) => CellState::Floor,
        None => panic!("Invalid cell in coordinates <{},{}>", row_idx, col_idx)
    }
}

/*
 * Returns the state of the next cell visible in the provided direction (specified by row
 * and column offsets), or None if no cell is visible in that direction. If `ignore_floor`
 * is true, CellState::Floor will never be returned, and instead the first non-floor cell
 * in that direction (or None) is returned. If no offset is specified (both are 0), None
 * is returned.
 */
fn get_visible_cell_in_direction(state: &State, row_idx: i32, col_idx: i32, row_diff: i32, col_diff: i32, ignore_floor: bool) -> Option<CellState> {
    if row_diff == 0 && col_diff == 0 {
        return None;
    }

    let next_row = row_idx + row_diff;
    let next_col = col_idx + col_diff;

    let next_cell_state = state.get_cell(next_row, next_col);

    if next_cell_state == Some(CellState::Floor) && ignore_floor {
        get_visible_cell_in_direction(state, next_row, next_col, row_diff, col_diff, ignore_floor)
    } else {
        next_cell_state
    }
}

fn parse_init_state(input: &[String]) -> State {
    State::new(input
        .iter()
        .map(|r| r.chars().map(CellState::new).collect())
        .collect())
}

#[derive(Eq, PartialEq, Clone)]
enum CellState {
    Empty,
    Occupied,
    Floor,
}

impl CellState {
    fn new(cell_char: char) -> CellState {
        match cell_char {
            '.' => CellState::Floor,
            'L' => CellState::Empty,
            '#' => CellState::Occupied,
            _ => panic!("Unidentified cell state: {}", cell_char),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            CellState::Floor => ".",
            CellState::Empty => "L",
            CellState::Occupied => "#",
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    state: Vec<Vec<CellState>>
}

impl State {
    fn new(cell_values: Vec<Vec<CellState>>) -> State {
        State {
            state: cell_values
        }
    }

    fn row_range(&self) -> Range<i32> {
        0..self.state.len() as i32
    }

    fn col_range(&self) -> Range<i32> {
        0..self.state[0].len() as i32
    }

    fn get_cell(&self, row: i32, col: i32) -> Option<CellState> {
        if self.row_range().contains(&row) && self.col_range().contains(&col) {
            Some(self.state[row as usize][col as usize].clone())
        } else {
            None
        }
    }

    fn get_state_count(&self, state: CellState) -> u32 {

        self.state
        .iter()
        .map(|r| r.iter().filter(|c| c == &&state).count())
        .sum::<usize>() as u32
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_folder = |row: &Vec<CellState>| row.iter().fold(String::new(), |acc, c| acc + c.to_string());
        let col_folder = |col: &Vec<Vec<CellState>>| col.iter().fold(String::new(), |acc, r| acc + &row_folder(r) + "\n");
        let output = col_folder(&self.state);
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    fn get_example() -> Vec<String> {
        vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ]
    }

    #[test]
    fn verify_example_task_1() {
        assert_eq!(37, crate::task_1(&get_example()));
    }

    #[test]
    fn verify_example_task_2() {
        assert_eq!(26, crate::task_2(&get_example()));
    }
}
