use std::ops::{Index, IndexMut};

fn read_input(input: &str) -> ([usize; 3], Vec<usize>) {
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .collect();

    let registers: Vec<usize> = lines[0..3]
        .iter()
        .map(|line| line
            .split_whitespace()
            .last().unwrap()
            .parse::<usize>().unwrap()
        ).collect();

    let program: Vec<usize> = lines[4]
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect();

    ([registers[0], registers[1], registers[2]], program)
}

#[derive(Clone, Copy)]
enum Register {
    A = 0,
    B,
    C
}

impl Register {
    fn from_operand(operand: usize) -> Self {
        match operand {
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid operand: {operand}.")
        }
    }
}

impl Index<Register> for [usize; 3] {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Register> for [usize; 3] {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

enum Instruction {
    ADV = 0,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

impl Instruction {
    fn from_opcode(opcode: usize) -> Self {
        match opcode {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Invalid opcode: {opcode}.")
        }
    }
}

fn combo_value(registers: &[usize; 3], operand: usize) -> usize {
    match operand {
        0..=3 => {
            operand
        },
        4..=6 => {
            registers[Register::from_operand(operand)]
        }
        _ => panic!("Invalid operand: {operand}.")
    }
}

fn run_program(mut registers: [usize; 3], program: &[usize], part_2: bool) -> Vec<usize> {
    let mut ip = 0;
    let mut jmp = false;

    let mut output: Vec<usize> = Vec::new();

    while ip < program.len() && (!part_2 || output.is_empty() || output.last() == program.get(output.len() - 1)) {

        match Instruction::from_opcode(program[ip]) {
            Instruction::ADV => registers[Register::A] >>= combo_value(&registers, program[ip + 1]),
            Instruction::BXL => registers[Register::B] ^= program[ip + 1],
            Instruction::BST => registers[Register::B] = combo_value(&registers, program[ip + 1]) % 8,
            Instruction::JNZ => if registers[Register::A] != 0 {
                ip = program[ip + 1];
                jmp = true;
            },
            Instruction::BXC => registers[Register::B] ^= registers[Register::C],
            Instruction::OUT => output.push(combo_value(&registers, program[ip + 1]) % 8),
            Instruction::BDV => registers[Register::B] = registers[Register::A] >> combo_value(&registers, program[ip + 1]),
            Instruction::CDV => registers[Register::C] = registers[Register::A] >> combo_value(&registers, program[ip + 1]),
        }

        if !jmp {
            ip += 2;
        } else {
            jmp = false;
        }
    }

    output
}

pub fn part1(input: &str) -> String {
    let (registers, program) = read_input(input);
    run_program(registers, &program, false)
        .into_iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn usize_to_bit_vec(value: usize, num_bits: usize) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in (0..num_bits).rev() {
        bits.push(((value >> i) & 1) as u8);
    }
    bits
}

fn bits_slice_to_usize(bits: &[u8]) -> usize {
    let mut value = 0;
    for (i, &bit) in bits.iter().rev().enumerate() {
        value += (bit as usize) << i;
    }
    value
}

/// Returns a list of rules the 10 bits starting at some position should adhere to for the program to print the instruction.
/// An item in such a list can take the value 0, 1 or 2.
/// A 2 indicates that both a 0 or a 1 is possible.
/// 
/// The program boils down to calculating (((a % 8) ^ 3) ^ (a >> ((a % 8) ^ 5))) % 8 and then shifting "a" right by 3 bits, stopping when "a" becomes 0.
fn options(instruction: usize) -> Vec<Vec<u8>> {
    let mut options = Vec::new();
    
    // For all 3 bit numbers, find all allowed 10 bit numbers suffixed by that 3 bit number.
    for i in 0..8 {
        // The binary representation of all values that are allowed when the first 3 bits are i.
        // At most 10 bits are required to calculate the output value.
        let mut rep = vec![2; 7];
        rep.extend(usize_to_bit_vec(i, 3));

        // This 3 bit number ensures that the result of the calculation is the program instruction.
        let enabler = usize_to_bit_vec(i ^ 3 ^ instruction, 3);
        let mut push = true;

        for (j, &e) in enabler.iter().enumerate() {
            // Place the enabler bits in the correct position given by the shift i ^ 5.
            let index = rep.len() - (i ^ 5) - 3 + j;

            // When the enabler bits overlap with the actual bits, we need to check whether they are the same.
            if index >= 7 && rep[index] != e {
                push = false;
                break;
            }

            rep[index] = e;
        }

        if push {
            options.push(rep);
        }
    }

    options
}

/// Recursively search for the smallest number such that the program prints itself.
/// 
/// The options are the options returned by the options function.
/// The ans is the binary representation of the answer, which is mutated throughout the recursion.
/// The program is the input program.
/// The program index indicates the current program instruction to test.
fn search(
    options: &[Vec<Vec<u8>>],
    ans: &mut [u8; 7 + 16 * 3],
    program: &[usize],
    program_index: usize,
) -> bool {
    if program_index == program.len() {
        return true;
    }

    // Determine the options that are compliant with the current answer.
    let compliant_options: Vec<&Vec<u8>> = options[program[program.len() - 1 - program_index]]
            .iter()
            .filter(|option| (0..7).all(|i| ans[i + 3 * program_index] == option[i] || option[i] == 2))
            .collect();

    // Store the current answer so we can reset it when the search fails in this branch.
    let old_ans = *ans;

    for option in compliant_options {
        // Prepare the answer for branching by setting the last three bits.
        for i in 0..3 {
            ans[i + 7 + 3 * program_index] = option[i + 7];
        }
        if search(options, ans, program, program_index + 1) {
            return true;
        }
    }

    // Reset the answer as we failed in this branch.
    *ans = old_ans;
    false
}

pub fn part2(input: &str) -> usize {
    let (_, program) = read_input(input);

    let options: Vec<Vec<Vec<u8>>> = (0..8).map(options).collect();
    let mut ans = [0; 7 + 16 * 3];

    search(&options, &mut ans, &program, 0);

    bits_slice_to_usize(&ans.into_iter().map(|bit| if bit == 1 { 1 } else { 0 }).collect::<Vec<u8>>())
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
        assert_eq!(part1("Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0"),
        String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(part2("Register A: 2024
    //     Register B: 0
    //     Register C: 0

    //     Program: 0,3,5,4,3,0"), 117440);
    // }
}
