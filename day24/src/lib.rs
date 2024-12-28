use std::collections::{HashMap, HashSet, VecDeque};

fn read_input(input: &str) -> (Vec<(&str, usize)>, Vec<Vec<&str>>) {
    let mut lines = input.lines();

    let mut input_bits = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let input_bit: Vec<&str> = line.split_whitespace().collect();
        input_bits.push((&input_bit[0][..input_bit[0].len() - 1], input_bit[1].parse().unwrap()));
    }

    let mut circuit = Vec::new();
    for line in lines {
        let split = line.split_whitespace().collect();
        circuit.push(split);
    }

    (input_bits, circuit)
}

type Dag<'a> = (HashMap<&'a str, HashSet<&'a str>>, HashMap<&'a str, HashSet<&'a str>>);
fn build_graph<'a>(circuit: &[Vec<&'a str>]) -> Dag<'a> {
    circuit
        .iter()
        .fold((HashMap::new(), HashMap::new()), |(mut outgoing, mut incoming), wire| {
            outgoing.entry(wire[0]).or_default().insert(wire[4]);
            outgoing.entry(wire[2]).or_default().insert(wire[4]);
            incoming.entry(wire[4]).or_default().insert(wire[0]);
            incoming.entry(wire[4]).or_default().insert(wire[2]);
            (outgoing, incoming)
        })
}

fn build_ordering<'a>(input_bits: &[(&'a str, usize)], (outgoing, mut incoming): Dag<'a>) -> HashMap<&'a str, usize> {
    let mut order_index = 0;
    let mut order = HashMap::new();

    let mut queue: VecDeque<&&str> = input_bits.iter().map(|(node, _)| node).collect();
    while let Some(&node) = queue.pop_front() {
        if order.contains_key(node) {
            continue;
        }
        
        order.insert(node, order_index);
        order_index += 1;

        if let Some(targets) = outgoing.get(node) {
            for target in targets.iter() {
                incoming.get_mut(target).unwrap().remove(node);
                if incoming[target].is_empty() {
                    queue.push_back(target);
                    incoming.remove(target);
                }
            }
        }
    }

    order
}

pub fn part1(input: &str) -> usize {
    let (input_bits, mut circuit) = read_input(input);

    let dag = build_graph(&circuit);
    let order = build_ordering(&input_bits, dag);
    circuit.sort_by(|a, b| order[a[4]].cmp(&order[b[4]]));
    
    let mut output_values: HashMap<&str, bool> = input_bits
        .into_iter()
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.insert(key, value == 1);
            map
        });

    for wire in circuit {
        output_values.insert(wire[4], match wire[1] {
            "AND" => output_values[wire[0]] && output_values[wire[2]],
            "OR" => output_values[wire[0]] || output_values[wire[2]],
            "XOR" => output_values[wire[0]] != output_values[wire[2]],
            _ => panic!("Invalid operation.")
        });
    }

    output_values
        .into_iter()
        .filter(|(key, value)| &key[0..1] == "z" && *value)
        .fold(0, |acc, (key, _)| {
            let index: usize = key[1..=2].parse().unwrap();
            acc + (1 << index)
        })
}

pub fn part2(input: &str) -> usize {
    let input = read_input(input);
    0
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
        "x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(get_input()), 2024);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(get_input()), 0);
    }
}
