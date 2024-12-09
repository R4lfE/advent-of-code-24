use std::collections::{HashMap, HashSet};

fn read_input(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, line)| {
            for (j, c) in line.trim().chars().enumerate() {
                if c.is_alphanumeric() {
                    map
                        .entry(c)
                        .or_default()
                        .push((i as i32, j as i32));
                }
            }
            map
        })
}

pub fn part1(input: &str) -> usize {
    let bound = input.lines().count() as i32;
    let input = read_input(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antennas in input.values() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (dy, dx) = (antennas[i].0 - antennas[j].0, antennas[i].1 - antennas[j].1);

                if 0 <= antennas[i].0 + dy && antennas[i].0 + dy < bound
                && 0 <= antennas[i].1 + dx && antennas[i].1 + dx < bound {
                    antinodes.insert((dy + antennas[i].0, dx + antennas[i].1));
                }

                if 0 <= antennas[j].0 - dy && antennas[j].0 - dy < bound
                && 0 <= antennas[j].1 - dx && antennas[j].1 - dx < bound {
                    antinodes.insert((antennas[j].0 - dy, antennas[j].1 - dx));
                }
            }
        }
    }

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let bound = input.lines().count() as i32;
    let input = read_input(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antennas in input.values() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (dy, dx) = (antennas[i].0 - antennas[j].0, antennas[i].1 - antennas[j].1);
                
                // Set to 0 to also include self.
                let mut k = 0;
                while 0 <= antennas[i].0 + k * dy && antennas[i].0 + k * dy < bound
                && 0 <= antennas[i].1 + k * dx && antennas[i].1 + k * dx < bound {
                    antinodes.insert((antennas[i].0 + k * dy, antennas[i].1 + k * dx));
                    k += 1;
                }

                k = 0;
                while 0 <= antennas[j].0 - k * dy && antennas[j].0 - k * dy < bound
                && 0 <= antennas[j].1 - k * dx && antennas[j].1 - k * dx < bound {
                    antinodes.insert((antennas[j].0 - k * dy, antennas[j].1 - k * dx));
                    k += 1;
                }
            }
        }
    }

    antinodes.len()
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
        "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 14);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 34);
    }
}
