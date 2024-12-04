fn read_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let input = read_input(input);
    todo!()
}

pub fn part2(input: &str) -> usize {
    let input = read_input(input);
    todo!()
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
        ""
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 0);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 0);
    }
}
