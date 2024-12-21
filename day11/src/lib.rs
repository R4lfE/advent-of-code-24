fn read_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
        ).collect()
}

pub fn part1_and_2(input: &str, iterations: usize) -> usize {
    let mut stones = read_input(input);

    for _ in 0..iterations {
        dbg!(&stones);
        for i in 0..stones.len() {
            // Stone engraved with 0 becomes 1.
            if stones[i] == 0 {
                stones[i] = 1;
            }
            // Stone with even number of digits is split.
            else if (stones[i] as f64).log10() as usize % 2 == 1 {
                let digits: String = stones[i].to_string();
                stones[i] = digits[..digits.len() / 2].parse().unwrap();
                stones.push(digits[digits.len() / 2..].parse().unwrap());
            }
            // Multiply the value by 2024.
            else {
                stones[i] *= 2024;
            }
        }
    }
    
    stones.len()
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::*;

    #[test]
    fn run() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input.txt")?;
        dbg!(part1_and_2(&input, 25));
        dbg!(part1_and_2(&input, 75));
        Ok(())
    }

    fn get_input<'a>() -> &'a str {
        "0 1 2 3 4 5 6 7 8 9"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1_and_2(get_input(), 8), 55312);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part1_and_2(get_input(), 75), 55312);
    }
}
