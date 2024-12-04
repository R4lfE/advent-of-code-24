fn read_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|s| s.trim().chars().collect())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let xmas = ['X', 'M', 'A', 'S'];
    let grid = read_grid(input);
    
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // Right.
            if j <= grid[i].len() - 4 && (0..4).all(|k| grid[i][j + k] == xmas[k]) {
                count += 1;
            }
            // Up right.
            if i >= 3 && j <= grid[i].len() - 4 && (0..4).all(|k| grid[i - k][j + k] == xmas[k]) {
                count += 1;
            }
            // Up.
            if i >= 3 && (0..4).all(|k| grid[i - k][j] == xmas[k]) {
                count += 1;
            }
            // Up left.
            if i >= 3 && j >= 3 && (0..4).all(|k| grid[i - k][j - k] == xmas[k]) {
                count += 1;
            }
            // Left.
            if j >= 3 && (0..4).all(|k| grid[i][j - k] == xmas[k]) {
                count += 1;
            }
            // Down left.
            if i <= grid.len() - 4 && j >= 3 && (0..4).all(|k| grid[i + k][j - k] == xmas[k]) {
                count += 1;
            }
            // Down.
            if i <= grid.len() - 4 && (0..4).all(|k| grid[i + k][j] == xmas[k]) {
                count += 1;
            }
            // Down right.
            if i <= grid.len() - 4 && j <= grid[i].len() - 4 && (0..4).all(|k| grid[i + k][j + k] == xmas[k]) {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    let mas = ['M', 'A', 'S'];
    let grid = read_grid(input);
    
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // Look for MAS in each diagonal centered at grid[i][j].
            if i >= 1 && i <= grid.len() - 2 && j >= 1 && j <= grid[i].len() - 2 
            && ((0..3).all(|k| grid[i - 1 + k][j - 1 + k] == mas[k])
                || (0..3).all(|k| grid[i - 1 + k][j - 1 + k] == mas[2 - k]))
            && ((0..3).all(|k| grid[i + 1 - k][j - 1 + k] == mas[k])
                || (0..3).all(|k| grid[i + 1 - k][j - 1 + k] == mas[2 - k])) {
                count += 1;
            }
        }
    }

    count
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
        "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 18);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 9);
    }
}
