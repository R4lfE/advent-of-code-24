fn read_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line
            .trim()
            .chars()
            .map(|c| c
                .to_digit(10)
                .unwrap() as usize
            ).collect::<Vec<usize>>())
        .collect()
}

fn checksum(rep: Vec<Option<usize>>) -> usize {
    let mut i = 0;
    let mut acc = 0;
    while i < rep.len() {
        if let Some(val) = rep[i] {
            acc += i * val;
        }
        i += 1;
    }
    acc
}

pub fn part1(input: &str) -> usize {
    let input = read_input(input);
    
    let mut rep: Vec<Option<usize>> = Vec::new();
    let mut spaces = 0;
    for (i, &count) in input.iter().enumerate() {
        if i % 2 == 0 {
            rep.extend(vec![Some(i / 2); count]);
            spaces += count;
        } else {
            rep.extend(vec![None; count]);
        }
    }

    let mut left = 0;
    let mut right = rep.len() - 1;

    while spaces < right {
        while rep[left].is_some() {
            left += 1;
        }
        while rep[right].is_none() {
            right -= 1;
        }
        rep[left] = rep[right];
        rep[right] = None;
    }
    
    checksum(rep)
}

pub fn part2(input: &str) -> usize {
    let input = read_input(input);
    
    let mut rep: Vec<Option<usize>> = Vec::new();
    for (i, &count) in input.iter().enumerate() {
        if i % 2 == 0 {
            rep.extend(vec![Some(i / 2); count]);
        } else {
            rep.extend(vec![None; count]);
        }
    }

    let mut right = rep.len() - 1;

    while right > 0 {
        while rep[right].is_none() {
            right -= 1;
        }

        let mut len = 0;
        let id = rep[right];
        while rep[right] == id {
            if right == 0 {
                break;
            }
            right -= 1;
            len += 1;
        }

        let mut left = 0;
        while left < right {
            while rep[left].is_some() {
                left += 1;
            }
            if left > right {
                break;
            }
            let mut free_len = 0;
            while rep[left].is_none() && free_len < len {
                free_len += 1;
                left += 1;
            }
            if free_len == len {
                for i in 1..=len {
                    rep[left - i] = id;
                    rep[right + i] = None;
                }
                break;
            }
        }
    }
    
    checksum(rep)
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
        "2333133121414131402"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 1928);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 2858);
    }
}
