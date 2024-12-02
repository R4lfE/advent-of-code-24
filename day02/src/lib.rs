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

pub fn part1(input: &str) -> usize {
    read_levels(input)
        .into_iter()
        .fold(0, |acc, level| acc + is_safe(level) as usize)
}

pub fn part2_bf(input: &str) -> usize {
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

pub fn part2(input: &str) -> usize {
    read_levels(input)
        .into_iter()
        .fold(0, |acc, level| {
            let mut failed = false;
            let mut i = 0;

            let mut left = level[0];
            let mut right = level[1];

            while i < level.len() - 1 {
                if (left < right && level[i] < level[i + 1] && level[i + 1] - level[i] <= 3)
                || (left > right && level[i] > level[i + 1] && level[i] - level[i + 1] <= 3) {
                    i += 1;
                } else if !failed {
                    failed = true;

                    // Test if we can omit i.
                    if i == 0 {
                        left = level[1];
                        right = level[2];
                    } else if i == 1 {
                        right = level[2];
                    }

                    if (
                        left < right
                        && (i == 0 || (level[i - 1] < level[i + 1] && level[i + 1] - level[i - 1] <= 3))
                        && (level.len() <= i + 2 || (level[i + 1] < level[i + 2] && level[i + 2] - level[i + 1] <= 3))
                        && (level.len() <= i + 3 || (level[i + 2] < level[i + 3] && level[i + 3] - level[i + 2] <= 3))
                    ) || (
                        left > right
                        && (i == 0 || (level[i - 1] > level[i + 1] && level[i - 1] - level[i + 1] <= 3))
                        && (level.len() <= i + 2 || (level[i + 1] > level[i + 2] && level[i + 1] - level[i + 2] <= 3))
                        && (level.len() <= i + 3 || (level[i + 2] > level[i + 3] && level[i + 2] - level[i + 3] <= 3))
                    ) {
                        i += 2;
                        continue;
                    }

                    // Test if we can omit i + 1.
                    if i == 0 {
                        left = level[0];
                        right = level[2];
                    } else if i == 1 {
                        right = level[1];
                    }

                    if (
                        left < right
                        && (level.len() <= i + 2 || (level[i] < level[i + 2] && level[i + 2] - level[i] <= 3))
                        && (level.len() <= i + 3 || (level[i + 2] < level[i + 3] && level[i + 3] - level[i + 2] <= 3))
                    ) || (
                        left > right
                        && (level.len() <= i + 2 || (level[i] > level[i + 2] && level[i] - level[i + 2] <= 3))
                        && (level.len() <= i + 3 || (level[i + 2] > level[i + 3] && level[i + 2] - level[i + 3] <= 3))
                    ) {
                        i += 2;
                        continue;
                    }

                    // Test if we can omit the first number to swap the direction.
                    if i == 1 {
                        left = level[1];
                        right = level[2];
                    }
                } else {
                    return acc;
                }
            }
            acc + 1
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
        dbg!(part2_bf(&input));
        dbg!(part2(&input));
        Ok(())
    }

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
        assert_eq!(part2_bf(input), 4);
        assert_eq!(part2(input), 4);
    }
}