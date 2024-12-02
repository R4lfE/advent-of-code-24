use std::{cmp::{max, min}, collections::HashMap};

fn read_lines(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
                let nums: Vec<usize> = line
                    .split_whitespace()
                    .map(|num_str| num_str
                        .parse::<usize>()
                        .expect("Parsed string is not a number.")
                    ).collect();
                (nums[0], nums[1])
        }).unzip()
}

pub fn part1(input: &str) -> usize {
    let (mut left, mut right) = read_lines(input);
    
    left.sort();
    right.sort();

    left
        .into_iter()
        .zip(right)
        .fold(0, |acc, (a, b)| acc + max(a, b) - min(a, b))
}

pub fn part2(input: &str) -> usize {
    let (left, right) = read_lines(input);

    let left_map = left
        .into_iter()
        .fold(HashMap::new(), |mut map, a| {
            *map.entry(a).or_insert(0) += 1;
            map
        });

    right
        .into_iter()
        .fold(0, |acc, b| match left_map.get(&b) {
            Some(a_count) => acc + b * a_count,
            None => acc
        })
}

#[cfg(test)]
mod test {
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
        "3   4
         4   3
         2   5
         1   3
         3   9
         3   3"
    }

    #[test]
    fn part1_test() {
        let input = get_input();
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part2_test() {
        let input = get_input();
        assert_eq!(part2(input), 31);
    }
}
