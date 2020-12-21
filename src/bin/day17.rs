use std::collections::{HashMap, HashSet};

mod helpers;

fn main() {
    let filename: &str = "day17.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u32 {
    let result = simulate(input, 3, 6);
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u32 {
    let result = simulate(input, 4, 6);
    println!("Task 2: {}", result);
    result
}

fn parse_state(input: &[String], dimensions: u8) -> HashSet<Point> {
    let mut active_cells: HashSet<Point> = HashSet::new();
    for x in 0..input.len() {
        for (y, state) in input[x].char_indices() {
            if state == '#' {
                active_cells.insert(match dimensions {
                    3 => Point::new_3d(x as i32, y as i32, 0),
                    4 => Point::new_4d(x as i32, y as i32, 0, 0),
                    _ => panic!("Unsupported dimensions: {}", dimensions)
                });
            }
        }
    }

    active_cells
}

fn simulate(initial_state: &[String], dimensions: u8, rounds: u8) -> u32 {
    let mut state = parse_state(initial_state, dimensions);

    for _ in 0..rounds {
        // Initialize the frequency map with current state's cubes (and 0 neighbours) so
        // they are guaranteed to appear in the results
        let mut frequencies: HashMap<Point, u8> = state.iter().cloned().map(|p| (p, 0)).collect();
        // Count the frequencies of different points as neighbours, i.e. get their
        // active neighbour counts
        for point in state.iter().flat_map(|c| c.get_neighbours()) {
            *frequencies.entry(point).or_insert(0) += 1;
        }

        let mut new_state: HashSet<Point> = HashSet::new();
        for (point, neighbours) in frequencies {
            if state.contains(&point) {
                if neighbours == 2 || neighbours == 3 {
                    new_state.insert(point);
                }
            } else {
                if neighbours == 3 {
                    new_state.insert(point);
                }
            }
        }
        state = new_state;
    }

    state.len() as u32
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    dimensions: i8,
}

impl Point {
    fn new_3d(x: i32, y: i32, z: i32) -> Point {
        Point {
            x,
            y,
            z,
            w: 0,
            dimensions: 3,
        }
    }

    fn new_4d(x: i32, y: i32, z: i32, w: i32) -> Point {
        Point {
            x,
            y,
            z,
            w,
            dimensions: 4,
        }
    }

    fn get_neighbours(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        for x in (self.x - 1)..=(self.x + 1) {
            for y in (self.y - 1)..=(self.y + 1) {
                for z in (self.z - 1)..=(self.z + 1) {
                    if self.dimensions == 3 {
                        // Coordinate is not its own neighbour
                        if x != self.x || y != self.y || z != self.z {
                            result.push(Point::new_3d(x, y, z));
                        }
                    } else if self.dimensions == 4 {
                        for w in (self.w - 1)..=(self.w + 1) {
                            if x != self.x || y != self.y || z != self.z || w != self.w {
                                result.push(Point::new_4d(x, y, z, w));
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        assert_eq!(112, crate::task_1(&input));
    }

    #[test]
    fn verify_example_task_2() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        assert_eq!(848, crate::task_2(&input));
    }
}
