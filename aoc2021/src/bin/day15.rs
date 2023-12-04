use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::HashSet;
use std::vec::Vec;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        15
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

struct HeightMaps {
    map: Vec<Vec<u8>>,
    riskmap: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl HeightMaps {
    pub fn new(input: &str) -> HeightMaps {
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut riskmap: Vec<Vec<u32>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<u8> = Vec::new();
            let mut riskrow: Vec<u32> = Vec::new();
            for risk in line.as_bytes().iter() {
                row.push(*risk - 48);
                riskrow.push(u32::MAX);
            }
            map.push(row);
            riskmap.push(riskrow);
        }
        riskmap[0][0] = 0u32;
        let width = map[0].len();
        let height = map.len();
        HeightMaps {
            map: map,
            riskmap: riskmap,
            width: width,
            height: height,
        }
    }

    pub fn expand(&mut self) {
        let (oldwidth, oldheight) = (self.width, self.height);
        self.width *= 5;
        self.height *= 5;
        for y in 0..self.height {
            if y >= self.map.len() {
                self.map.push(Vec::new());
                self.riskmap.push(Vec::new());
            }
            for x in 0..self.width {
                if x < oldwidth && y < oldheight {
                    // This is a spot on the original tile..
                    continue;
                }
                let original = self.map[y % oldheight][x % oldwidth];
                let increase = (x / oldwidth) + (y / oldheight);
                let mut newval = original + increase as u8;
                if newval > 9 {
                    newval = (newval % 10) + 1;
                }
                self.map[y].push(newval);
                self.riskmap[y].push(u32::MAX);
            }
        }
    }

    // fn printmap(&self) {
    //   for y in 0..self.map.len() {
    //     for x in 0..self.map[0].len() {
    //       print!("{:02} ", self.map[y][x]);
    //     }
    //     println!("");
    //   }
    // }

    // fn printrisk(&self) {
    //   for y in 0..self.riskmap.len() {
    //     for x in 0..self.riskmap[0].len() {
    //       print!("{:02} ", self.riskmap[y][x]);
    //     }
    //     println!("");
    //   }
    // }

    // pub fn djikstra(&mut self) -> u32 {
    //     // Initialize the unvisited set to all nodes.
    //     let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             unvisited.insert((x, y));
    //         }
    //     }
    //     // Initialize the visited set to be empty.
    //     let mut visited: HashSet<(usize, usize)> = HashSet::new();
    //     // Initialize current node to the start node (upper left).
    //     let mut current = (0usize, 0usize);
    //     // Loop until the end node (bottom right) is in the visited set.
    //     while !visited.contains(&(self.width - 1, self.height - 1)) {
    //         // Add unvisited neighbors to the tentative set
    //         let mut tentative: Vec<(usize, usize)> = Vec::new();
    //         if current.0 > 0 && !visited.contains(&(current.0 - 1, current.1)) {
    //             tentative.push((current.0 - 1, current.1));
    //         }
    //         if current.1 > 0 && !visited.contains(&(current.0, current.1 - 1)) {
    //             tentative.push((current.0, current.1 - 1));
    //         }
    //         if current.0 < self.width - 1 && !visited.contains(&(current.0 + 1, current.1)) {
    //             tentative.push((current.0 + 1, current.1));
    //         }
    //         if current.1 < self.height - 1 && !visited.contains(&(current.0, current.1 + 1)) {
    //             tentative.push((current.0, current.1 + 1));
    //         }

    //         // Update the risk for each neighbor in the tentative set.
    //         let current_risk = self.riskmap[current.1][current.0];
    //         for (x, y) in &tentative {
    //             let edge_risk = self.map[*y][*x] as u32;
    //             let tentative_risk = self.riskmap[*y][*x];
    //             if edge_risk + current_risk < tentative_risk {
    //                 self.riskmap[*y][*x] = edge_risk + current_risk;
    //             }
    //         }
    //         visited.insert(current.clone());
    //         unvisited.remove(&current);

    //         // Find the unvisited node with the shortest tentative distance.
    //         let mut shortest_neighbor = (0usize, 0usize);
    //         let mut shortest = u32::MAX;
    //         for (x, y) in &unvisited {
    //             if self.riskmap[*y][*x] < shortest {
    //                 shortest_neighbor = (*x, *y);
    //                 shortest = self.riskmap[*y][*x];
    //             }
    //         }

    //         // Make the shortest neighbor the new current node.
    //         current = shortest_neighbor;
    //     }
    //     self.riskmap[self.map.len() - 1][self.map[0].len() - 1]
    // }

    // Heuristic guess at the shortest path from n to the end: take the
    // manhattan distance between n and the end.
    fn astar_h(&self, n: (usize, usize)) -> u32 {
        ((self.width - n.0 - 1) + (self.height - n.1 - 1)) as u32
    }

    pub fn astar(&mut self) -> u32 {
        let mut open: HashSet<(usize, usize)> = HashSet::new();
        open.insert((0, 0));
        let mut guessmap = self.riskmap.clone();
        guessmap[0][0] = self.astar_h((0, 0));
        while !open.is_empty() {
            let mut current = (0usize, 0usize);
            let mut shortest = u32::MAX;
            for (x, y) in &open {
                if guessmap[*y][*x] < shortest {
                    current = (*x, *y);
                    shortest = guessmap[*y][*x];
                }
            }
            if current == (self.width - 1, self.height - 1) {
                break;
            }
            open.remove(&current);

            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            if current.0 > 0 {
                neighbors.push((current.0 - 1, current.1));
            }
            if current.1 > 0 {
                neighbors.push((current.0, current.1 - 1));
            }
            if current.0 < self.width - 1 {
                neighbors.push((current.0 + 1, current.1));
            }
            if current.1 < self.height - 1 {
                neighbors.push((current.0, current.1 + 1));
            }
            for neighbor in neighbors {
                let tentative =
                    self.riskmap[current.1][current.0] + self.map[neighbor.1][neighbor.0] as u32;
                if tentative < self.riskmap[neighbor.1][neighbor.0] {
                    self.riskmap[neighbor.1][neighbor.0] = tentative;
                    guessmap[neighbor.1][neighbor.0] = tentative + self.astar_h(neighbor);
                    open.insert(neighbor);
                }
            }
        }
        self.riskmap[self.map.len() - 1][self.map[0].len() - 1]
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut maps = HeightMaps::new(input);
    maps.astar()
}

pub fn part_two(input: &str) -> u32 {
    let mut maps = HeightMaps::new(input);
    maps.expand();
    println!("map expanded");
    maps.astar()
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ),
            40
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ),
            315
        );
    }
}
