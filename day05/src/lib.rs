use std::collections::{HashMap, HashSet, VecDeque};

fn read_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let input: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .collect();
    let split: Vec<&[&str]> = input.split(|line| line.is_empty()).collect();

    let rules: Vec<(usize, usize)> = split[0]
        .iter()
        .map(|line| {
            let nums: Vec<usize> = line
                .split('|')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            (nums[0], nums[1])
        }).collect();

    let updates: Vec<Vec<usize>> = split[1]
        .iter()
        .map(|line| line
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect()
        ).collect();
    
    (rules, updates)
}

fn update_subgraph(rules: &[(usize, usize)], update: &[usize]) -> Vec<(usize, usize)> {
    let update: HashSet<usize> = update.iter().copied().collect();
    rules
        .iter()
        .copied()
        .filter(|(source, target)| update.contains(source) && update.contains(target))
        .collect()
}

fn get_order(update: &[usize]) -> HashMap<usize, usize> {
    update
        .iter()
        .copied()
        .enumerate()
        .map(|(i, num)| (num ,i))
        .collect()
}

fn is_sorted(rules: &[(usize, usize)], order: &HashMap<usize, usize>) -> bool {
    rules.iter().all(|(s, t)| order[s] < order[t])
}

pub fn part1(input: &str) -> usize {
    let (rules, updates) = read_input(input);
    updates
        .into_iter()
        .fold(0, |acc, update| {
            let order = get_order(&update);
            if is_sorted(&update_subgraph(&rules, &update), &order) {
                acc + update[update.len() / 2]
            } else {
                acc
            }
        })
}

/// Topological sort from wikipedia.
fn center_of_sorted_update(rules: &[(usize, usize)], center_index: usize) -> usize {
    let mut outgoing: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .fold(HashMap::new(), |mut map, rule| {
            map.entry(rule.0).or_default().insert(rule.1);
            map
        });

    let mut incoming: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .fold(HashMap::new(), |mut map, rule| {
            map.entry(rule.1).or_default().insert(rule.0);
            map
        });

    let (sources, targets): (HashSet<usize>, HashSet<usize>) = rules.iter().copied().unzip();
    let mut queue: VecDeque<usize> = sources.difference(&targets).copied().collect();

    let mut sorted_update: Vec<usize> = Vec::new();

    while let Some(page) = queue.pop_front() {
        sorted_update.push(page);
        if sorted_update.len() - 1 == center_index {
            return *sorted_update.last().unwrap();
        }

        if let Some(neighbors) = outgoing.get(&page) {
            for neighbor in neighbors.iter() {
                // Remove the edge from the page coming into the neighbor.
                incoming.get_mut(neighbor).unwrap().remove(&page);
                if incoming[neighbor].is_empty() {
                    queue.push_back(*neighbor);
                }
            }
        }
        // Remove all outgoing edges from the current page.
        outgoing.remove(&page);
    }

    0
}

pub fn part2(input: &str) -> usize {
    let (rules, updates) = read_input(input);
    updates
        .into_iter()
        .fold(0, |acc, update| {
            let order = get_order(&update);
            let subgraph = update_subgraph(&rules, &update);
            if !is_sorted(&subgraph, &order) {
                acc + center_of_sorted_update(&subgraph, update.len() / 2)
            } else {
                acc
            }
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
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 143);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 123);
    }
}
