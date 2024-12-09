#[derive(Clone)]
enum Operator {
    Add,
    Mul,
    Cat
}

fn read_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.trim().split(':').collect();
            (
                split[0].parse().unwrap(),
                split[1]
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            )
        }).collect()
}

fn eval(target: usize, numbers: &[usize], operators: &[Operator]) -> bool {
    let mut acc = numbers[0];
    for i in 0..operators.len() {
        if acc > target {
            break;
        }

        acc = match operators[i] {
            Operator::Add => acc + numbers[i + 1],
            Operator::Mul => acc * numbers[i + 1],
            Operator::Cat => acc * 10usize.pow(numbers[i + 1].to_string().len() as u32) + numbers[i + 1],
        };
    }
    target == acc
}

pub fn part1(input: &str) -> usize {
    let input = read_input(input);

    input
        .into_iter()
        .fold(0, |acc, (target, numbers)| {
            let len = numbers.len() - 1;
            let mut operators = vec![Operator::Add; len];

            loop {
                if eval(target, &numbers, &operators) {
                    return acc + target;
                }

                let mut carry = true;
                for operator in operators.iter_mut() {
                    if carry {
                        match operator {
                            Operator::Add => {
                                *operator = Operator::Mul;
                                carry = false;
                                break;
                            },
                            Operator::Mul => {
                                *operator = Operator::Add;
                                carry = true;
                            },
                            Operator::Cat => {
                                panic!("Unreachable.")
                            },
                        }
                    }
                }

                if carry {
                    break;
                }
            }
            acc
        })
}

pub fn part2(input: &str) -> usize {
    let input = read_input(input);

    input
        .into_iter()
        .fold(0, |acc, (target, numbers)| {
            let len = numbers.len() - 1;
            let mut operators = vec![Operator::Add; len];

            loop {
                if eval(target, &numbers, &operators) {
                    return acc + target;
                }

                let mut carry = true;
                for operator in operators.iter_mut() {
                    if carry {
                        match operator {
                            Operator::Add => {
                                *operator = Operator::Mul;
                                carry = false;
                                break;
                            },
                            Operator::Mul => {
                                *operator = Operator::Cat;
                                carry = false;
                                break;
                            },
                            Operator::Cat => {
                                *operator = Operator::Add;
                                carry = true;
                            },
                        }
                    }
                }

                if carry {
                    break;
                }
            }
            acc
        })
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
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 3749);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 11387);
    }
}
