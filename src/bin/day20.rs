use std::collections::{HashSet, VecDeque};
mod helpers;

const SEAMONSTER_PATTERN: [[bool; 20]; 3] = [
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, false,
    ],
    [
        true, false, false, false, false, true, true, false, false, false, false, true, true,
        false, false, false, false, true, true, true,
    ],
    [
        false, true, false, false, true, false, false, true, false, false, true, false, false,
        true, false, false, true, false, false, false,
    ],
];

fn main() {
    let filename: &str = "day20.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u64 {
    let tiles = (0..(input.len() + 1) / 12)
        .map(|idx| Tile::parse(&input[12 * idx..12 * idx + 11]))
        .collect::<Vec<Tile>>();
    let placed_tiles = place_tiles(tiles);

    let width = placed_tiles[0].len();
    let height = placed_tiles.len();
    let result = placed_tiles[0][0].id
        * placed_tiles[0][width - 1].id
        * placed_tiles[height - 1][0].id
        * placed_tiles[height - 1][width - 1].id;
    println!("Task 1: {}", result);
    result
}

fn task_2(input: &[String]) -> u64 {
    let tiles = (0..(input.len() + 1) / 12)
        .map(|idx| Tile::parse(&input[12 * idx..12 * idx + 11]))
        .collect::<Vec<Tile>>();
    let placed_tiles = place_tiles(tiles);

    let monster_size = SEAMONSTER_PATTERN
        .iter()
        .map(|r| r.iter().filter(|v| **v).count())
        .sum::<usize>();

    let mut result = 0;
    // Force the bitmap to a square by filling with empty rows so the orientation function
    // works as expected
    let bitmap = tiles_to_bitmap(&placed_tiles);

    for orientation in &Orientation::all() {
        let flipped_bitmap = orient_bitmap(&bitmap, &orientation);
        let monster_count = count_seamonsters(&flipped_bitmap);
        if monster_count > 0 {
            let all_nonempty_pixels = flipped_bitmap
                .iter()
                .map(|r| r.iter().filter(|v| **v).count())
                .sum::<usize>();
            result = all_nonempty_pixels - monster_count * monster_size;
        }
    }
    println!("Task 2: {}", result);
    result as u64
}

fn place_tiles(tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
    // Build a large grid for placing the tiles
    const P_SIDE: usize = 100;
    let orientations = Orientation::all();
    let mut p: Vec<Vec<Option<Tile>>> = vec![vec![Option::None; P_SIDE]; P_SIDE];

    let mut tiles_to_place = VecDeque::from(tiles);
    let mut search_frontier: HashSet<(usize, usize)> = HashSet::new();
    search_frontier.insert((P_SIDE / 2 - 1, P_SIDE / 2 - 1));

    'outer: while let Some(curr_tile) = tiles_to_place.pop_front() {
        for o in &orientations {
            let oriented_tile = curr_tile.orient(o);

            for pos in search_frontier.clone() {
                if can_be_placed(&p, &oriented_tile, &pos) {
                    p[pos.0][pos.1] = Some(oriented_tile);
                    update_search_frontier(&p, &pos, &mut search_frontier);
                    continue 'outer;
                }
            }
        }
        // Tile could not be placed, push it back to the queue
        tiles_to_place.push_back(curr_tile);
    }

    p.iter()
        .map(|r| r.iter().flatten().cloned().collect::<Vec<Tile>>())
        .filter(|r| !r.is_empty())
        .collect()
}

fn update_search_frontier<T>(
    area: &Vec<Vec<Option<T>>>,
    current_cell: &(usize, usize),
    search_frontier: &mut HashSet<(usize, usize)>,
) {
    for offset_pos in &[
        (current_cell.0, current_cell.1 - 1),
        (current_cell.0 - 1, current_cell.1),
        (current_cell.0, current_cell.1),
        (current_cell.0 + 1, current_cell.1),
        (current_cell.0, current_cell.1 + 1),
    ] {
        if area[offset_pos.0][offset_pos.1].is_some() {
            search_frontier.remove(&offset_pos);
        } else {
            search_frontier.insert(*offset_pos);
        }
    }
}

fn can_be_placed(area: &Vec<Vec<Option<Tile>>>, tile: &Tile, position: &(usize, usize)) -> bool {
    for (own_side, neighbor_side, neighbor_coords) in &[
        (Side::North, Side::South, (position.0, position.1 - 1)),
        (Side::East, Side::West, (position.0 + 1, position.1)),
        (Side::South, Side::North, (position.0, position.1 + 1)),
        (Side::West, Side::East, (position.0 - 1, position.1)),
    ] {
        if let Some(neighbour) = &area[neighbor_coords.0][neighbor_coords.1] {
            if tile.get_side(*own_side) != neighbour.get_side(*neighbor_side) {
                return false;
            }
        }
    }
    true
}

fn tiles_to_bitmap(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<bool>> {
    const CONTENT_SIDE: usize = 8;

    let mut result: Vec<Vec<bool>> = Vec::new();

    for tile_row in 0..tiles.len() {
        for row in 1..CONTENT_SIDE + 1 {
            let mut current_row: Vec<bool> = Vec::new();
            for tile_col in 0..tiles[0].len() {
                for col in 1..CONTENT_SIDE + 1 {
                    current_row.push(tiles[tile_row][tile_col].values[col][row]);
                }
            }
            result.push(current_row);
        }
    }

    result
}

fn orient_bitmap(bitmap: &Vec<Vec<bool>>, orientation: &Orientation) -> Vec<Vec<bool>> {
    let mut results: Vec<Vec<bool>> = Vec::new();
    let height = bitmap.len();
    let width = bitmap[0].len();

    for i in 0..height {
        let ni = height - 1 - i;

        let mut curr_row: Vec<bool> = Vec::new();
        for j in 0..width {
            let nj = width - 1 - j;
            if orientation.flipped {
                curr_row.push(match orientation.rotate_direction {
                    Side::North => bitmap[i][nj],
                    Side::East => bitmap[nj][ni],
                    Side::South => bitmap[ni][j],
                    Side::West => bitmap[j][i],
                });
            } else {
                curr_row.push(match orientation.rotate_direction {
                    Side::North => bitmap[i][j],
                    Side::East => bitmap[j][ni],
                    Side::South => bitmap[ni][nj],
                    Side::West => bitmap[nj][i],
                })
            }
        }
        results.push(curr_row);
    }

    results
}

fn count_seamonsters(input: &Vec<Vec<bool>>) -> usize {
    let mut seamonster_count = 0;

    for y in 0..(input.len() - SEAMONSTER_PATTERN.len()) {
        for x in 0..(input[0].len() - SEAMONSTER_PATTERN[0].len()) {
            // (i,j) is the offset for the seamonster pattern
            let mut pattern_matched = true;
            'outer: for j in 0..SEAMONSTER_PATTERN.len() {
                for i in 0..SEAMONSTER_PATTERN[0].len() {
                    if SEAMONSTER_PATTERN[j][i] && !input[y + j][x + i] {
                        pattern_matched = false;
                        break 'outer;
                    }
                }
            }
            if pattern_matched {
                seamonster_count += 1;
            }
        }
    }
    seamonster_count
}

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    values: Vec<Vec<bool>>,
    orientation: Orientation,
}

impl Tile {
    fn parse(input: &[String]) -> Tile {
        let id = input[0]
            .trim_start_matches("Tile ")
            .trim_end_matches(":")
            .parse::<u64>()
            .unwrap();
        // Cells are always 10x10
        let values = input[1..=10]
            .iter()
            .map(|r| r.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();

        Tile {
            id,
            values,
            orientation: Orientation {
                rotate_direction: Side::North,
                flipped: false,
            },
        }
    }

    fn orient(&self, orientation: &Orientation) -> Tile {
        Tile {
            id: self.id,
            values: orient_bitmap(&self.values, &orientation),
            orientation: orientation.clone(),
        }
    }

    fn get_side(&self, side: Side) -> Vec<bool> {
        match side {
            Side::North => self.values[0].clone(),
            Side::East => self.values.iter().map(|r| r[r.len() - 1]).collect(),
            Side::South => self.values[self.values.len() - 1].clone(),
            Side::West => self.values.iter().map(|r| r[0]).collect(),
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum Side {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Copy)]
struct Orientation {
    rotate_direction: Side,
    flipped: bool,
}

impl Orientation {
    fn all() -> [Orientation; 8] {
        [
            Orientation {
                rotate_direction: Side::North,
                flipped: false,
            },
            Orientation {
                rotate_direction: Side::East,
                flipped: false,
            },
            Orientation {
                rotate_direction: Side::South,
                flipped: false,
            },
            Orientation {
                rotate_direction: Side::West,
                flipped: false,
            },
            Orientation {
                rotate_direction: Side::North,
                flipped: true,
            },
            Orientation {
                rotate_direction: Side::East,
                flipped: true,
            },
            Orientation {
                rotate_direction: Side::South,
                flipped: true,
            },
            Orientation {
                rotate_direction: Side::West,
                flipped: true,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_day20_example_task_1() {
        // The sample input is so large it's in a separate file
        let input = &crate::helpers::input_helpers::read_input("day20_example.txt").unwrap();
        assert_eq!(20899048083289, crate::task_1(&input));
    }

    #[test]
    fn verify_day20_example_task_2() {
        // The sample input is so large it's in a separate file
        let input = &crate::helpers::input_helpers::read_input("day20_example.txt").unwrap();

        assert_eq!(273, crate::task_2(&input));
    }
}
