use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        10
    }

    fn part_one(&self, input: &str) -> String {
        let pipes = parse_matrix(input);
        let mut start = (0usize, 0usize);

        // Find the starting point.
        // Initialize our two search pointers to the starting point.
        {
            let mut found = false;
            for row in 0..pipes.len() {
                for col in 0..pipes[row].len() {
                    if pipes[row][col] == 'S' {
                        start = (row, col);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }

        // Find the initial two directions we're going in.
        let mut ptr1 = (0usize, 0usize);
        let mut ptr2 = (0usize, 0usize);
        let mut dir1 = Direction::NORTH;
        let mut dir2 = Direction::NORTH;

        let mut found1 = false;
        let mut found2 = false;
        if start.0 != 0 {
            // Not at the top, so check north.
            match pipes[start.0 - 1][start.1] {
                '|' => {
                    ptr1 = (start.0 - 1, start.1);
                    dir1 = Direction::NORTH;
                    found1 = true;
                }
                '7' => {
                    ptr1 = (start.0 - 1, start.1);
                    dir1 = Direction::WEST;
                    found1 = true;
                }
                'F' => {
                    ptr1 = (start.0 - 1, start.1);
                    dir1 = Direction::EAST;
                    found1 = true;
                }
                _ => (),
            }
        }
        if start.1 != 0 {
            // Not in the first column, so check west.
            match pipes[start.0][start.1 - 1] {
                '-' => {
                    if found1 {
                        ptr2 = (start.0, start.1 - 1);
                        dir2 = Direction::WEST;
                        found2 = true;
                    } else {
                        ptr1 = (start.0, start.1 - 1);
                        dir1 = Direction::WEST;
                        found1 = true;
                    }
                }
                'L' => {
                    if found1 {
                        ptr2 = (start.0, start.1 - 1);
                        dir2 = Direction::NORTH;
                        found2 = true;
                    } else {
                        ptr1 = (start.0, start.1 - 1);
                        dir1 = Direction::NORTH;
                        found1 = true;
                    }
                }
                'F' => {
                    if found1 {
                        ptr2 = (start.0, start.1 - 1);
                        dir2 = Direction::SOUTH;
                        found2 = true;
                    } else {
                        ptr1 = (start.0, start.1 - 1);
                        dir1 = Direction::SOUTH;
                        found1 = true;
                    }
                }
                _ => (),
            }
        }
        if start.0 != pipes.len() - 1 && !found2 {
            // Not in the bottom, so check south.
            match pipes[start.0 + 1][start.1] {
                '|' => {
                    if found1 {
                        ptr2 = (start.0 + 1, start.1);
                        dir2 = Direction::SOUTH;
                        found2 = true;
                    } else {
                        ptr1 = (start.0 + 1, start.1);
                        dir1 = Direction::SOUTH;
                        found1 = true;
                    }
                }
                'L' => {
                    if found1 {
                        ptr2 = (start.0 + 1, start.1);
                        dir2 = Direction::EAST;
                        found2 = true;
                    } else {
                        ptr1 = (start.0 + 1, start.1);
                        dir1 = Direction::EAST;
                        found1 = true;
                    }
                }
                'J' => {
                    if found1 {
                        ptr2 = (start.0 + 1, start.1);
                        dir2 = Direction::WEST;
                        found2 = true;
                    } else {
                        ptr1 = (start.0 + 1, start.1);
                        dir1 = Direction::WEST;
                        found1 = true;
                    }
                }
                _ => (),
            }
        }
        if start.1 != pipes[0].len() - 1 && !found2 {
            // Not in the last column, so check east.
            match pipes[start.0][start.1 + 1] {
                '-' => {
                    ptr2 = (start.0, start.1 + 1);
                    dir2 = Direction::EAST;
                    found2 = true;
                }
                'J' => {
                    ptr2 = (start.0, start.1 + 1);
                    dir2 = Direction::NORTH;
                    found2 = true;
                }
                '7' => {
                    ptr2 = (start.0, start.1 + 1);
                    dir2 = Direction::SOUTH;
                    found2 = true;
                }
                _ => (),
            }
        }

        if !found1 || !found2 {
            panic!("Didn't find the two starting points for our pointers");
        }

        // At this point, we can just follow the path until the pointers meet.
        let mut dist = 1usize;
        while ptr1 != ptr2 {
            match dir1 {
                Direction::NORTH => {
                    ptr1.0 -= 1; // Move ptr1 north.
                    dir1 = match pipes[ptr1.0][ptr1.1] {
                        '|' => Direction::NORTH,
                        '7' => Direction::WEST,
                        'F' => Direction::EAST,
                        _ => panic!("ptr1: Unexpected character after moving north"),
                    }; // Get the new direction.
                }
                Direction::SOUTH => {
                    ptr1.0 += 1; // Move ptr1 south.
                    dir1 = match pipes[ptr1.0][ptr1.1] {
                        '|' => Direction::SOUTH,
                        'J' => Direction::WEST,
                        'L' => Direction::EAST,
                        _ => panic!("ptr1: Unexpected character after moving south"),
                    }; // Get the new direction.
                }
                Direction::WEST => {
                    ptr1.1 -= 1; // Move ptr1 west.
                    dir1 = match pipes[ptr1.0][ptr1.1] {
                        '-' => Direction::WEST,
                        'F' => Direction::SOUTH,
                        'L' => Direction::NORTH,
                        _ => panic!("ptr1: Unexpected character after moving west"),
                    }; // Get the new direction.
                }
                Direction::EAST => {
                    ptr1.1 += 1; // Move ptr1 east.
                    dir1 = match pipes[ptr1.0][ptr1.1] {
                        '-' => Direction::EAST,
                        'J' => Direction::NORTH,
                        '7' => Direction::SOUTH,
                        _ => panic!("ptr1: Unexpected character after moving east"),
                    }; // Get the new direction.
                }
            }

            if ptr1 == ptr2 {
                // The were next to each other.
                break;
            }

            // If they weren't next to each other, update ptr2.
            match dir2 {
                Direction::NORTH => {
                    ptr2.0 -= 1; // Move ptr2 north.
                    dir2 = match pipes[ptr2.0][ptr2.1] {
                        '|' => Direction::NORTH,
                        '7' => Direction::WEST,
                        'F' => Direction::EAST,
                        _ => panic!("ptr2: Unexpected character after moving north"),
                    }; // Get the new direction.
                }
                Direction::SOUTH => {
                    ptr2.0 += 1; // Move ptr2 south.
                    dir2 = match pipes[ptr2.0][ptr2.1] {
                        '|' => Direction::SOUTH,
                        'J' => Direction::WEST,
                        'L' => Direction::EAST,
                        _ => panic!("ptr2: Unexpected character after moving south"),
                    }; // Get the new direction.
                }
                Direction::WEST => {
                    ptr2.1 -= 1; // Move ptr2 west.
                    dir2 = match pipes[ptr2.0][ptr2.1] {
                        '-' => Direction::WEST,
                        'F' => Direction::SOUTH,
                        'L' => Direction::NORTH,
                        _ => panic!("ptr2: Unexpected character after moving west"),
                    }; // Get the new direction.
                }
                Direction::EAST => {
                    ptr2.1 += 1; // Move ptr2 east.
                    dir2 = match pipes[ptr2.0][ptr2.1] {
                        '-' => Direction::EAST,
                        'J' => Direction::NORTH,
                        '7' => Direction::SOUTH,
                        _ => panic!("ptr2: Unexpected character after moving east"),
                    }; // Get the new direction.
                }
            }

            dist += 1;
        }
        dist.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut pipes = parse_matrix(input);
        let mut start = (0usize, 0usize);

        // Find the starting point.
        // Initialize our two search pointers to the starting point.
        let mut found = false;
        for row in 0..pipes.len() {
            for col in 0..pipes[row].len() {
                if pipes[row][col] == 'S' {
                    start = (row, col);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        // Find the initial direction we're going in.
        let mut ptr = (0usize, 0usize);
        let mut dir = Direction::NORTH;
        let mut start_out_dir = dir;
        found = false;
        if start.0 != 0 {
            // Not at the top, so check north.
            match pipes[start.0 - 1][start.1] {
                '|' => {
                    ptr = (start.0 - 1, start.1);
                    dir = Direction::NORTH;
                    found = true;
                }
                '7' => {
                    ptr = (start.0 - 1, start.1);
                    dir = Direction::WEST;
                    found = true;
                }
                'F' => {
                    ptr = (start.0 - 1, start.1);
                    dir = Direction::EAST;
                    found = true;
                }
                _ => (),
            }
            if found {
                start_out_dir = Direction::NORTH;
            }
        }
        if start.1 != 0 && !found {
            // Not in the first column, so check west.
            match pipes[start.0][start.1 - 1] {
                '-' => {
                    ptr = (start.0, start.1 - 1);
                    dir = Direction::WEST;
                    found = true;
                }
                'L' => {
                    ptr = (start.0, start.1 - 1);
                    dir = Direction::NORTH;
                    found = true;
                }
                'F' => {
                    ptr = (start.0, start.1 - 1);
                    dir = Direction::SOUTH;
                    found = true;
                }
                _ => (),
            }
            if found {
                start_out_dir = Direction::WEST;
            }
        }
        if start.0 != pipes.len() - 1 && !found {
            // Not in the bottom, so check south.
            match pipes[start.0 + 1][start.1] {
                '|' => {
                    ptr = (start.0 + 1, start.1);
                    dir = Direction::SOUTH;
                    found = true;
                }
                'L' => {
                    ptr = (start.0 + 1, start.1);
                    dir = Direction::EAST;
                    found = true;
                }
                'J' => {
                    ptr = (start.0 + 1, start.1);
                    dir = Direction::WEST;
                    found = true;
                }
                _ => (),
            }
            if found {
                start_out_dir = Direction::SOUTH;
            }
        }
        if start.1 != pipes[0].len() - 1 && !found {
            // Not in the last column, so check east.
            match pipes[start.0][start.1 + 1] {
                '-' => {
                    ptr = (start.0, start.1 + 1);
                    dir = Direction::EAST;
                    found = true;
                }
                'J' => {
                    ptr = (start.0, start.1 + 1);
                    dir = Direction::NORTH;
                    found = true;
                }
                '7' => {
                    ptr = (start.0, start.1 + 1);
                    dir = Direction::SOUTH;
                    found = true;
                }
                _ => (),
            }
            if found {
                start_out_dir = Direction::EAST;
            }
        }

        if !found {
            panic!("Didn't find the starting point for our pointer");
        }

        // At this point, we can just follow the path until the pointers meet.  At each spot,
        // replace the character with a B to mark it as the "border" of the loop.
        let mut start_in_dir = dir;
        while ptr != start {
            pipes[ptr.0][ptr.1] = match pipes[ptr.0][ptr.1] {
                '-' => '_',
                '|' => 'I',
                'L' => 'l',
                'J' => 'j',
                'F' => 'f',
                '7' => '&',
                _ => panic!("Unexpected pipe character"),
            };
            let newdir: Direction;
            match dir {
                Direction::NORTH => {
                    ptr.0 -= 1; // Move ptr north.
                    if pipes[ptr.0][ptr.1] == 'S' {
                        start_in_dir = dir;
                        break;
                    }
                    newdir = match pipes[ptr.0][ptr.1] {
                        '|' => Direction::NORTH,
                        '7' => Direction::WEST,
                        'F' => Direction::EAST,
                        _ => panic!("ptr: Unexpected character after moving north"),
                    }; // Get the new direction.
                }
                Direction::SOUTH => {
                    ptr.0 += 1; // Move ptr south.
                    if pipes[ptr.0][ptr.1] == 'S' {
                        start_in_dir = dir;
                        break;
                    }
                    newdir = match pipes[ptr.0][ptr.1] {
                        '|' => Direction::SOUTH,
                        'J' => Direction::WEST,
                        'L' => Direction::EAST,
                        _ => panic!("ptr: Unexpected character after moving south"),
                    }; // Get the new direction.
                }
                Direction::WEST => {
                    ptr.1 -= 1; // Move ptr west.
                    if pipes[ptr.0][ptr.1] == 'S' {
                        start_in_dir = dir;
                        break;
                    }
                    newdir = match pipes[ptr.0][ptr.1] {
                        '-' => Direction::WEST,
                        'F' => Direction::SOUTH,
                        'L' => Direction::NORTH,
                        _ => panic!("ptr: Unexpected character after moving west"),
                    }; // Get the new direction.
                }
                Direction::EAST => {
                    ptr.1 += 1; // Move ptr east.
                    if pipes[ptr.0][ptr.1] == 'S' {
                        start_in_dir = dir;
                        break;
                    }
                    newdir = match pipes[ptr.0][ptr.1] {
                        '-' => Direction::EAST,
                        'J' => Direction::NORTH,
                        '7' => Direction::SOUTH,
                        _ => panic!("ptr: Unexpected character after moving east"),
                    }; // Get the new direction.
                }
            }
            dir = newdir;
        }

        // Make sure the intersection got marked.
        if (start_in_dir == Direction::EAST || start_in_dir == Direction::WEST)
            && start_in_dir == start_out_dir
        {
            pipes[start.0][start.1] = '_';
        } else if (start_in_dir == Direction::NORTH || start_in_dir == Direction::SOUTH)
            && start_in_dir == start_out_dir
        {
            pipes[start.0][start.1] = 'I';
        } else if (start_in_dir == Direction::EAST && start_out_dir == Direction::NORTH)
            || (start_in_dir == Direction::SOUTH && start_out_dir == Direction::WEST)
        {
            pipes[start.0][start.1] = 'j';
        } else if (start_in_dir == Direction::WEST && start_out_dir == Direction::NORTH)
            || (start_in_dir == Direction::SOUTH && start_out_dir == Direction::EAST)
        {
            pipes[start.0][start.1] = 'l';
        } else if (start_in_dir == Direction::WEST && start_out_dir == Direction::SOUTH)
            || (start_in_dir == Direction::NORTH && start_out_dir == Direction::EAST)
        {
            pipes[start.0][start.1] = 'f';
        } else if (start_in_dir == Direction::EAST && start_out_dir == Direction::SOUTH)
            || (start_in_dir == Direction::NORTH && start_out_dir == Direction::WEST)
        {
            pipes[start.0][start.1] = '&';
        }

        let mut count = 0usize;
        for row in pipes.iter() {
            let mut crosses = 0usize;
            let mut last_elbow = '\0';
            for val in row {
                match val {
                    'I' => crosses += 1,
                    'j' => {
                        if last_elbow != 'f' {
                            crosses += 1
                        }
                        last_elbow = 'j';
                    }
                    '&' => {
                        if last_elbow != 'l' {
                            crosses += 1
                        }
                        last_elbow = '&';
                    }
                    'l' => {
                        crosses += 1;
                        last_elbow = 'l';
                    }
                    'f' => {
                        crosses += 1;
                        last_elbow = 'f';
                    }
                    '_' => (),
                    _ => {
                        if crosses % 2 == 1 {
                            count += 1;
                        }
                    }
                }
            }
        }

        count.to_string()
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for val in row {
            print!("{}", val);
        }
        println!("");
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "4"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            "4"
        );
        assert_eq!(
            solution.part_two(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            "10"
        );
    }
}
