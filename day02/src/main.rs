use std::{error::Error, fs};

fn read_levels(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|num| num
                .parse::<usize>()
                .expect("Parsed string is not a number.")
            ).collect()
        ).collect()
}

fn is_safe(level: Vec<usize>) -> bool {
    level
        .windows(2)
        .all(|window| 
            (level[0] < level[1] && window[0] < window[1] && window[1] - window[0] <= 3)
            || (level[0] > level[1] && window[0] > window[1] && window[0] - window[1] <= 3)
        )
}

fn part1(input: &str) -> usize {
    read_levels(input)
        .into_iter()
        .fold(0, |acc, level| acc + is_safe(level) as usize)
}

fn part2(input: &str) -> usize {
    read_levels(input)
        .into_iter()
        .fold(0, |acc, level| {
            (0..level.len())
                .any(|i| {
                    let mut cln = level.clone();
                    cln.remove(i);
                    is_safe(cln)
                }) as usize
            + acc
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    dbg!(part1(&input));
    dbg!(part2(&input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
       "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"
    }

    #[test]
    fn part1_test() {
        let input = get_input();
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_test() {
        let input = get_input();
        assert_eq!(part2(input), 4);
    }
}