use std::collections::{HashMap, HashSet};
mod helpers;

fn main() {
    let filename: &str = "day24.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u32 {
    let result = get_initial_state(input).len() as u32;
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u32 {
    let mut black_tiles = get_initial_state(input);

    // Simulate the state change rules 100 times
    for _ in 0..100 {
        let mut black_neighbour_counts: HashMap<(i8, i8, i8), u8> = HashMap::new();

        // Count the black neighbour cells
        for black_tile in &black_tiles {
            // If the tile is not already in the counter collection, add it so they are
            // correctly removed if no neighbours are found
            if !black_neighbour_counts.contains_key(&black_tile) {
                black_neighbour_counts.insert(black_tile.clone(), 0);
            }

            for neighbour in get_neighbours(&black_tile) {
                *black_neighbour_counts.entry(neighbour).or_insert(0) += 1;
            }
        }

        // Flip the necessary cells
        for (hex, count) in black_neighbour_counts {
            if black_tiles.contains(&hex) {
                if count == 0 || count > 2 {
                    black_tiles.remove(&hex);
                }
            } else {
                if count == 2 {
                    black_tiles.insert(hex);
                }
            }
        }
    }

    let result = black_tiles.len() as u32;
    println!("Task 2: {}", result);
    result
}

fn get_initial_state(input: &[String]) -> HashSet<(i8, i8, i8)> {
    let mut flipped_tiles: HashSet<(i8, i8, i8)> = HashSet::new();
    for r in input {
        let target_hex = instruction_to_offset(r);
        if flipped_tiles.contains(&target_hex) {
            flipped_tiles.remove(&target_hex);
        } else {
            flipped_tiles.insert(target_hex);
        }
    }
    flipped_tiles
}

/*
 * Return the cube-based coordinates of the target hex given the movement instructions
 * from the origin, as presented here: https://www.redblobgames.com/grids/hexagons/
 */
fn instruction_to_offset(input: &String) -> (i8, i8, i8) {
    let input_chars = input.chars().collect::<Vec<char>>();
    let mut current_coords = (0, 0, 0);
    let mut current_idx = 0;

    while current_idx < input_chars.len() {
        match input_chars[current_idx] {
            'n' => {
                current_idx += 1;
                match input_chars[current_idx] {
                    'e' => {
                        current_coords.0 += 1;
                        current_coords.2 -= 1;
                    }
                    'w' => {
                        current_coords.1 += 1;
                        current_coords.2 -= 1;
                    }
                    _ => panic!(
                        "Unexpected direction at index {} in: {}",
                        current_idx, input
                    ),
                }
            }
            'e' => {
                current_coords.0 += 1;
                current_coords.1 -= 1;
            }
            's' => {
                current_idx += 1;
                match input_chars[current_idx] {
                    'e' => {
                        current_coords.1 -= 1;
                        current_coords.2 += 1;
                    }
                    'w' => {
                        current_coords.0 -= 1;
                        current_coords.2 += 1;
                    }
                    _ => panic!(
                        "Unexpected direction at index {} in: {}",
                        current_idx, input
                    ),
                }
            }
            'w' => {
                current_coords.0 -= 1;
                current_coords.1 += 1;
            }
            _ => panic!(
                "Unexpected direction at index {} in: {}",
                current_idx, input
            ),
        };
        current_idx += 1;
    }
    current_coords
}

fn get_neighbours(cell: &(i8, i8, i8)) -> Vec<(i8, i8, i8)> {
    vec![
        (cell.0 + 1, cell.1 - 1, cell.2),
        (cell.0 + 1, cell.1, cell.2 - 1),
        (cell.0 - 1, cell.1 + 1, cell.2),
        (cell.0 - 1, cell.1, cell.2 + 1),
        (cell.0, cell.1 + 1, cell.2 - 1),
        (cell.0, cell.1 - 1, cell.2 + 1),
    ]
}

#[cfg(test)]
mod tests {

    fn get_example() -> Vec<String> {
        vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ]
    }

    #[test]
    fn verify_day24_example_task_1() {
        let input = get_example();
        assert_eq!(10, crate::task_1(&input));
    }

    #[test]
    fn verify_day24_example_task_2() {
        let input = get_example();
        assert_eq!(2208, crate::task_2(&input));
    }
}
