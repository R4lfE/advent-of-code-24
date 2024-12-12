use std::collections::{HashSet, VecDeque};

fn read_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        ).collect()
}

pub fn part1(input: &str) -> usize {
    let input = read_input(input);

    let mut score = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            // Found a trail head.
            if input[i][j] == 0 {
                // Find all trail ends with a BFS.
                let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(i, j)]);
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                while let Some((i_, j_)) = queue.pop_front() {
                    if visited.contains(&(i_, j_)) {
                        continue;
                    }
                    visited.insert((i_, j_));

                    let height = input[i_][j_];
                    if height == 9 {
                        score += 1;
                    } else {
                        if j_ < input[i_].len() - 1 && input[i_][j_ + 1] == input[i_][j_] + 1 {
                            queue.push_back((i_, j_ + 1));
                        }
                        if i_ > 0 && input[i_ - 1][j_] == input[i_][j_] + 1 {
                            queue.push_back((i_ - 1, j_));
                        }
                        if j_ > 0 && input[i_][j_ - 1] == input[i_][j_] + 1 {
                            queue.push_back((i_, j_ - 1));
                        }
                        if i_ < input.len() - 1 && input[i_ + 1][j_] == input[i_][j_] + 1 {
                            queue.push_back((i_ + 1, j_));
                        }
                    }
                }
            }
        }
    }
        
    score
}

pub fn part2(input: &str) -> usize {
    let input = read_input(input);

    let mut rating = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            // Found a trail head.
            if input[i][j] == 0 {
                // Find all trail ends with a BFS, allow duplicate heights.
                let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(i, j)]);
                while let Some((i_, j_)) = queue.pop_front() {
                    let height = input[i_][j_];
                    if height == 9 {
                        rating += 1;
                    } else {
                        if j_ < input[i_].len() - 1 && input[i_][j_ + 1] == input[i_][j_] + 1 {
                            queue.push_back((i_, j_ + 1));
                        }
                        if i_ > 0 && input[i_ - 1][j_] == input[i_][j_] + 1 {
                            queue.push_back((i_ - 1, j_));
                        }
                        if j_ > 0 && input[i_][j_ - 1] == input[i_][j_] + 1 {
                            queue.push_back((i_, j_ - 1));
                        }
                        if i_ < input.len() - 1 && input[i_ + 1][j_] == input[i_][j_] + 1 {
                            queue.push_back((i_ + 1, j_));
                        }
                    }
                }
            }
        }
    }
        
    rating
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
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 36);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 81);
    }
}
