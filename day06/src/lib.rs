#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '^' => Direction::Up,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Invalid direction character.")
        }
    }

    fn turn(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Space {
    Guard,
    Empty,
    Obstruction
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '>' | '^' | '<' | 'v' => Space::Guard,
            '.' => Space::Empty,
            '#' => Space::Obstruction,
            _ => panic!("Invalid space character.")
        }
    }
}

enum Move {
    Step(usize, usize),
    Turn
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn get_movement(map: &[Vec<char>], dir: &Direction, i: usize, j: usize) -> Option<Move> {
    (if j < map[0].len() - 1 && matches!(dir, Direction::Right) {
        Some((i, j + 1))
    } else if i > 0 && matches!(dir, Direction::Up) {
        Some((i - 1, j))
    } else if j > 0 && matches!(dir, Direction::Left) {
        Some((i, j - 1))
    } else if i < map.len() - 1 && matches!(dir, Direction::Down) {
        Some((i + 1, j))
    } else {
        None
    }).map(|(i_next, j_next)| match Space::from_char(map[i_next][j_next]) {
        Space::Obstruction => Move::Turn,
        _ => Move::Step(i_next, j_next)
    })
}

pub fn part1(input: &str) -> usize {
    let map = read_input(input);

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let (mut i, mut j) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row
            .iter()
            .position(|c| matches!(Space::from_char(*c), Space::Guard))
            .map(|j| (i, j)))
        .unwrap();

    let mut dir = Direction::from_char(map[i][j]);
    visited[i][j] = true;
    let mut count = 1;
    
    while let Some(movement) = get_movement(&map, &dir, i, j) {
        match movement {
            Move::Step(i_next, j_next) => {
                i = i_next;
                j = j_next;
                if !visited[i][j] {
                    visited[i][j] = true;
                    count += 1;
                }
            },
            Move::Turn => {
                dir = dir.turn();
            },
        }
    }

    count
}

fn find_cycle(map: &mut [Vec<char>], mut dir: Direction, mut i: usize, mut j: usize, visited: & mut[Vec<Vec<bool>>]) -> bool {
    let (i_obstacle, j_obstacle) = match dir {
        Direction::Right => (i, j + 1),
        Direction::Up => (i - 1, j),
        Direction::Left => (i, j - 1),
        Direction::Down => (i + 1, j),
    };

    // The obstacle cannot be placed in a space we have already visited.
    if visited[i_obstacle][j_obstacle].iter().any(|&vis| vis) {
        return false;
    }

    // Place obstacle.
    map[i_obstacle][j_obstacle] = '#';

    // Keep track of which positions we step during the cycle search so we may remove them when returning.
    let mut steps: Vec<(usize, usize, usize)> = Vec::new();

    while let Some(movement) = get_movement(map, &dir, i, j) {
        match movement {
            Move::Step(i_next, j_next) => {
                i = i_next;
                j = j_next;
            },
            Move::Turn => {
                dir = dir.turn();
            },
        }

        if visited[i][j][dir as usize] {
            // Reset old state and return cycle found.
            map[i_obstacle][j_obstacle] = '.';
            for (i, j, dir) in steps {
                visited[i][j][dir] = false;
            }
            return true;
        }

        steps.push((i, j, dir as usize));
        visited[i][j][dir as usize] = true;
    }

    // Reset old state and return cycle not found.
    map[i_obstacle][j_obstacle] = '.';
    for (i, j, dir) in steps {
        visited[i][j][dir] = false;
    }
    false
}

pub fn part2(input: &str) -> usize {
    let mut map = read_input(input);

    let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    let (mut i, mut j) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row
            .iter()
            .position(|c| matches!(Space::from_char(*c), Space::Guard))
            .map(|j| (i, j)))
        .unwrap();

    let mut dir = Direction::from_char(map[i][j]);
    visited[i][j][dir as usize] = true;
    let mut count = 0;
    
    while let Some(movement) = get_movement(&map, &dir, i, j) {
        if !matches!(movement, Move::Turn) && find_cycle(&mut map, dir, i, j, &mut visited) {
            count += 1;
        }

        match movement {
            Move::Step(i_next, j_next) => {
                i = i_next;
                j = j_next;
            },
            Move::Turn => {
                dir = dir.turn();
            },
        }

        visited[i][j][dir as usize] = true;
    }

    count
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::*;

    #[test]
    fn run() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input.txt")?;
        dbg!(part1(&input));
        dbg!(part2(&input));
        Ok(())
    }

    fn get_input<'a>() -> &'a str {
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 41);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 6);
    }
}
