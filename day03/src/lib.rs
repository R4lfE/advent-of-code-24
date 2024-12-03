use regex::Regex;

pub fn part1(input: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d)+,(\d)+\)").unwrap();
    mul_regex
        .find_iter(input)
        .fold(0, |acc, m| {
            let vals: Vec<usize> = m
                .as_str()[4..m.len() - 1]
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            acc + vals[0] * vals[1]
        })
}

pub fn part2(input: &str) -> usize {
    let input = format!("do(){input}");
    let input = input.as_str();

    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don\'t\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let range_starts: Vec<usize> = do_regex.find_iter(input).map(|m| m.end()).collect();
    let range_ends: Vec<usize> = dont_regex.find_iter(input).map(|m| m.start()).collect();

    // Merge ranges and remove duplicates.
    let mut ranges: Vec<(usize, bool)> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < range_starts.len() && j < range_ends.len() {
        if range_starts[i] < range_ends[j] {
            ranges.push((range_starts[i], true));
            i += 1;
        } else {
            ranges.push((range_ends[j], false));
            j += 1;
        }
    }

    ranges.extend(range_starts.iter().skip(i).map(|&start| (start, true)));
    ranges.extend(range_ends.iter().skip(j).map(|&end| (end, false)));
    ranges.dedup_by(|a, b| a.1 == b.1);

    let mut range_index = 0;
    mul_regex
        .find_iter(input)
        .map(|m| {
            let vals: Vec<usize> = m
                .as_str()[4..m.as_str().len() - 1]
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            (m.start(), vals[0] * vals[1])
        })
        .filter(|&(start, _)| {
            while range_index < ranges.len() && start > ranges[range_index].0 {
                range_index += 1;
            }
            range_index > 0 && ranges[range_index - 1].1
        })
        .map(|(_, result)| result)
        .sum()
}

pub fn part2_single_run(input: &str) -> usize {
    let input = format!("do(){}", input);

    let mul_regex = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut allow_mul = false;
    let mut result = 0;
    let mut pos = 0;

    while let Some(next) = input[pos..].find(|c| c == 'd' || c == 'm') {
        let match_start = pos + next;

        // Test do.
        if input[match_start..].starts_with("do()") {
            allow_mul = true;
            pos = match_start + 4;
        }
        // Test don't.
        else if input[match_start..].starts_with(r"don't()") {
            allow_mul = false;
            pos = match_start + 7;
        }
        // Test mul.
        else if let Some(mat) = mul_regex.find(&input[match_start..]) {
            if allow_mul {
                let caps = mul_regex.captures(mat.as_str()).unwrap();
                let x: usize = caps[1].parse().unwrap();
                let y: usize = caps[2].parse().unwrap();
                result += x * y;
            }
            pos = match_start + mat.end();
        } else {
            pos += 1;
        }
    }

    result
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

    #[test]
    fn part1_test() {
        assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
        assert_eq!(part2_single_run("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}
