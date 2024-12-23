use std::collections::{HashMap, HashSet};

fn read_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    (lines[0].split(',').map(|pattern| pattern.trim()).collect(),
    lines[1..].iter().map(|design| design.trim()).collect())
}

fn memoized(patterns: &Vec<&str>, design: &str, cache: &mut HashSet<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    
    if cache.contains(design) {
        return false;
    }

    for pattern in patterns.iter() {
        if let Some(remaining_design) = design.strip_prefix(pattern) {
            if memoized(patterns, remaining_design, cache) {
                return true;
            }
        }
    }

    cache.insert(design.to_string());
    false
}

pub fn part1(input: &str) -> usize {
    let (patterns, designs) = read_input(input);
    designs
        .iter()
        .filter(|design| memoized(&patterns, design, &mut HashSet::new()))
        .count()
}

fn memoized_counting(patterns: &Vec<&str>, design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    
    if let Some(&count) = cache.get(design) {
        return count;
    }

    for pattern in patterns.iter() {
        if let Some(remaining_design) = design.strip_prefix(pattern) {
            *cache.entry(design.to_string()).or_default() += memoized_counting(patterns, remaining_design, cache);
        }
    }

    *cache.entry(design.to_string()).or_default()
}

pub fn part2(input: &str) -> usize {
    let (patterns, designs) = read_input(input);
    designs
        .iter()
        .fold(0, |acc, design| 
            acc + memoized_counting(&patterns, design, &mut HashMap::new())
        )
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
        "r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 6);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 16);
    }
}
