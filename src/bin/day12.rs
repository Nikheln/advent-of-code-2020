mod helpers;

fn main() {
    let filename: &str = "day12.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();
    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u32 {
    let starting_position = Coordinates::new(0, 0);
    let instructions: Vec<Instruction1> = input.iter().map(Instruction1::parse).collect();
    let mut ship = Ship::new(starting_position.clone());

    for instruction in instructions {
        ship.process1(instruction);
    }
    let result = Coordinates::manhattan_distance(starting_position, ship.position);

    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u32 {
    let starting_position = Coordinates::new(0, 0);
    let instructions: Vec<Instruction2> = input.iter().map(Instruction2::parse).collect();
    let mut ship = Ship::new(starting_position.clone());

    for instruction in instructions {
        ship.process2(instruction);
    }
    let result = Coordinates::manhattan_distance(starting_position, ship.position);

    println!("Task 2: {}", result);
    result
}


enum Instruction1 {
    MoveToDirection(Direction, i32),
    MoveForward(i32),
    TurnLeft(i32),
    TurnRight(i32),
}

impl Instruction1 {
    fn parse(instruction: &String) -> Instruction1 {
        let (dir, arg) = instruction.split_at(1);
        let arg_num = arg.parse::<i32>().unwrap();

        match dir {
            "N" => Instruction1::MoveToDirection(Direction::North, arg_num),
            "S" => Instruction1::MoveToDirection(Direction::South, arg_num),
            "E" => Instruction1::MoveToDirection(Direction::East, arg_num),
            "W" => Instruction1::MoveToDirection(Direction::West, arg_num),
            "F" => Instruction1::MoveForward(arg_num),
            "L" => Instruction1::TurnLeft(arg_num),
            "R" => Instruction1::TurnRight(arg_num),
            &_ => panic!("Unidentified instruction: {}", instruction),
        }
    }
}

enum Instruction2 {
    MoveWaypointToDirection(Direction, i32),
    RotateWaypointLeft(i32),
    RotateWaypointRight(i32),
    MoveToWaypoint(i32),
}

impl Instruction2 {
    fn parse(instruction: &String) -> Instruction2 {
        let (dir, arg) = instruction.split_at(1);
        let arg_num = arg.parse::<i32>().unwrap();

        match dir {
            "N" => Instruction2::MoveWaypointToDirection(Direction::North, arg_num),
            "S" => Instruction2::MoveWaypointToDirection(Direction::South, arg_num),
            "E" => Instruction2::MoveWaypointToDirection(Direction::East, arg_num),
            "W" => Instruction2::MoveWaypointToDirection(Direction::West, arg_num),
            "L" => Instruction2::RotateWaypointLeft(arg_num),
            "R" => Instruction2::RotateWaypointRight(arg_num),
            "F" => Instruction2::MoveToWaypoint(arg_num),
            &_ => panic!("Unidentified instruction: {}", instruction),
        }
    }
}

#[derive(Clone)]
struct Coordinates {
    x: i32,
    y: i32
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }

    fn offset(&self, direction: Direction, distance: i32) -> Coordinates {
        match direction {
            Direction::North => self.offset_by(0, distance),
            Direction::South => self.offset_by(0, -distance),
            Direction::East => self.offset_by(distance, 0),
            Direction::West => self.offset_by(-distance, 0),
        }
    }

    fn offset_by(&self, x_offset: i32, y_offset: i32) -> Coordinates {
        Coordinates {
            x: self.x + x_offset,
            y: self.y + y_offset,
        }
    }

    fn rotate(&self, degrees: i32) -> Coordinates {
        let left_turns = (degrees / 90).rem_euclid(4);
        let (mut new_x, mut new_y) = (self.x, self.y);
        for _ in 0..left_turns {
            // Swap the coordinates, and negate the correct one
            let swp = new_y;
            new_y = new_x;
            new_x = swp * -1;
        }

        Coordinates::new(new_x, new_y)
    }

    fn manhattan_distance(a: Coordinates, b: Coordinates) -> u32 {
        ((b.x - a.x).abs() + (b.y - a.y).abs()) as u32
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, degrees: i32) -> Direction {
        let directions = [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ];

        let curr_idx = directions
            .iter()
            .position(|d| d == self)
            .unwrap();
        let new_idx = (curr_idx as i32 + (degrees / 90)).rem_euclid(4);

        directions[new_idx as usize]
    }
}

struct Ship {
    position: Coordinates,
    wp_rel_pos: Coordinates,
    orientation: Direction,
}

impl Ship {
    fn new(position: Coordinates) -> Ship {
        Ship {
            wp_rel_pos: position.offset_by(10, 1),
            position,
            orientation: Direction::East,
        }
    }

    fn process1(&mut self, input: Instruction1) {
        match input {
            Instruction1::MoveToDirection(direction, distance) => {
                self.position = self.position.offset(direction, distance)
            }
            Instruction1::MoveForward(distance) => {
                self.position = self.position.offset(self.orientation.clone(), distance)
            }
            Instruction1::TurnLeft(degrees) => self.orientation = self.orientation.turn(degrees),
            Instruction1::TurnRight(degrees) => self.orientation = self.orientation.turn(-degrees),
        };
    }

    fn process2(&mut self, input: Instruction2) {
        match input {
            Instruction2::MoveWaypointToDirection(direction, distance) => {
                self.wp_rel_pos = self.wp_rel_pos.offset(direction, distance);
            },
            Instruction2::MoveToWaypoint(count) => {
                self.position = self.position.offset_by(self.wp_rel_pos.x * count, self.wp_rel_pos.y * count);
            },
            Instruction2::RotateWaypointLeft(degrees) => {
                self.wp_rel_pos = self.wp_rel_pos.rotate(degrees);
            },
            Instruction2::RotateWaypointRight(degrees) => {
                self.wp_rel_pos = self.wp_rel_pos.rotate(-degrees);
            }
        };
    }
}

#[cfg(test)]
mod tests {

    fn get_example() -> Vec<String> {
        vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]
    }

    #[test]
    fn verify_example_task_1() {
        assert_eq!(25, crate::task_1(&get_example()));
    }

    #[test]
    fn verify_example_task_2() {
        assert_eq!(286, crate::task_2(&get_example()));
    }
}
