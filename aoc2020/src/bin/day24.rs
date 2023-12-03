use aoc_helpers::*;
use std::collections::HashMap;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        24
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

// False is white, true is black.
type Tiles = HashMap<Coord, bool>;

// Hex grid represented using "odd-r" horizontal offset coordinates.  This is
// effectively a square grid representation where the odd rows are shunted to
// the right.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    row: i64,
    col: i64,
}

pub fn dir_to_coord(start: &Coord, dir: &str) -> Coord {
    match dir {
        "e" => Coord {
            row: start.row,
            col: start.col - 1,
        },
        "se" => {
            if start.row % 2 == 0 {
                // Even row
                Coord {
                    row: start.row + 1,
                    col: start.col - 1,
                }
            } else {
                // Odd row
                Coord {
                    row: start.row + 1,
                    col: start.col,
                }
            }
        }
        "ne" => {
            if start.row % 2 == 0 {
                // Even row
                Coord {
                    row: start.row - 1,
                    col: start.col - 1,
                }
            } else {
                // Odd row
                Coord {
                    row: start.row - 1,
                    col: start.col,
                }
            }
        }
        "w" => Coord {
            row: start.row,
            col: start.col + 1,
        },
        "sw" => {
            if start.row % 2 == 0 {
                // Even row
                Coord {
                    row: start.row + 1,
                    col: start.col,
                }
            } else {
                // Odd row
                Coord {
                    row: start.row + 1,
                    col: start.col + 1,
                }
            }
        }
        "nw" => {
            if start.row % 2 == 0 {
                // Even row
                Coord {
                    row: start.row - 1,
                    col: start.col,
                }
            } else {
                // Odd row
                Coord {
                    row: start.row - 1,
                    col: start.col + 1,
                }
            }
        }
        _ => panic!("Invalid direction {}", dir),
    }
}

pub fn all_neighbors(start: &Coord) -> [Coord; 6] {
    [
        dir_to_coord(start, "e"),
        dir_to_coord(start, "ne"),
        dir_to_coord(start, "se"),
        dir_to_coord(start, "w"),
        dir_to_coord(start, "nw"),
        dir_to_coord(start, "sw"),
    ]
}

pub fn count_black_neighbors(start: &Coord, tiles: &Tiles) -> u32 {
    all_neighbors(start).iter().fold(0, |acc, coord| {
        if let Some(t) = tiles.get(coord) {
            if *t {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

// Returns true if tile flips from white to black, false if it flips from black
// to white.
pub fn walk_line(line: &str, tiles: &mut Tiles) -> bool {
    let mut i = 0;
    let mut tile = Coord { row: 0, col: 0 };
    while i < line.len() {
        let first_char = &line[i..(i + 1)];
        if first_char == "e" || first_char == "w" {
            tile = dir_to_coord(&tile, first_char);
            i += 1;
        } else {
            tile = dir_to_coord(&tile, &line[i..(i + 2)]);
            i += 2;
        }
    }
    // We "or_insert" true since the first time we touch a tile will involve
    // flipping it from white to black.
    let color = tiles.entry(tile).and_modify(|e| *e = !(*e)).or_insert(true);
    *color
}

pub fn part_one(input: &str) -> u32 {
    let mut tiles = Tiles::new();
    let mut count = 0;
    for line in input.lines() {
        if walk_line(line, &mut tiles) {
            count += 1;
        } else {
            count -= 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> i64 {
    // First, we repeat the logic from part 1 to get the day 0 state initialized.
    let mut tiles = Tiles::new();
    let mut count = 0;
    for line in input.lines() {
        if walk_line(line, &mut tiles) {
            count += 1;
        } else {
            count -= 1;
        }
    }
    // Next, we do 100 iterations for each following day, reading and updating the
    // tiles.
    for _ in 0..100 {
        let keys: Vec<Coord> = tiles.keys().cloned().collect();
        for k in keys {
            // Add neighbors for all the existing black tiles (if they don't already
            // exist).  We skip white tiles since they only change based on the
            // presence of black tiles, and any black tile neighbors are already
            // guaranteed to be in the map.
            if !(*tiles.entry(k).or_insert(false)) {
                continue;
            }
            let neighbors = all_neighbors(&k);
            for n in &neighbors {
                tiles.entry(*n).or_insert(false);
            }
        }
        let mut new_tiles = tiles.clone();
        for tile in tiles.keys() {
            let num_black = count_black_neighbors(tile, &tiles);
            let color = new_tiles.entry(*tile).or_insert(false);
            if *color && (num_black == 0 || num_black > 2) {
                *color = false;
                count -= 1;
            } else if !(*color) && num_black == 2 {
                *color = true;
                count += 1;
            }
        }
        tiles = new_tiles;
    }
    count
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            ),
            10
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            ),
            2208
        );
    }
}
