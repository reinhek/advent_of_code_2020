use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref REG_MEM: Regex = Regex::new(r"\[(?P<mem>[0-9]*)] = (?P<write>[0-9]*)").unwrap();
    }

    input.lines().map(|line| {
        return if line.starts_with("mask") {
            Instruction::parse(line.split_whitespace().last().unwrap())
        } else {
            let cap = REG_MEM.captures(line).unwrap();
            let address = (&cap[1]).parse().unwrap();
            let value = (&cap[2]).parse().unwrap();
            Instruction::Write(address, value)
        }
    }).collect()
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask([Bit; 36]),
    Write(usize, usize)
}

impl Instruction {
    fn parse(string: &str) -> Instruction {
        let mut mask = [Bit::Zero; 36];
        for (i, c) in string.chars().enumerate() {
            match c {
                '1' => mask[i] = Bit::One,
                '0' => mask[i] = Bit::Zero,
                'X' => mask[i] = Bit::X,
                _ => panic!("Error parsing bit"),
            }
        }
        Instruction::Mask(mask)
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    One,
    Zero,
    X
}

fn int_to_bin(mut integer: usize) -> [Bit;36] {
    let mut result = [Bit::Zero; 36];

    for iter in result.iter_mut().rev() {
        let bit = integer & 0x1 > 0;
        *iter = if bit {
            Bit::One
        } else {
            Bit::Zero
        };
        integer >>=1;
    }

    result
}

fn bin_to_int(binary: [Bit;36]) -> usize {
    let mut result = 0;
    for bit in binary.iter() {
        result = (result << 1) + if *bit == Bit::One {
            1
        } else {0};
    }
    result
}

fn permute(mut memory: Vec<[Bit;36]>) -> Vec<[Bit;36]> {
    let mut mem = Vec::new();
    while let Some(mut array) = memory.pop() {
        let mut changed = false;
        for i in 0..36 {
            if array[i] == Bit::X {
                array[i] = Bit::One;
                memory.push(array);

                array[i] = Bit::Zero;
                memory.push(array);

                changed = true;
                break;
            }
        }

        if !changed {
            mem.push(array);
        }
    }
    mem
}

trait Sum {
    fn sum(&self) -> usize;
}

impl Sum for HashMap<usize, [Bit;36]> {
    fn sum(&self) -> usize {
        let mut sum = 0;
        for mem in self.values() {
            sum += bin_to_int(*mem);
        }
        sum
    }
}

fn execute(instructions: &Vec<Instruction>, part: u8) -> usize {
    let mut memory: HashMap<usize, [Bit;36]>= HashMap::new();
    let mut mask = [Bit::Zero;36];

    for instruction in instructions {
        match instruction {
            Instruction::Mask(x) => mask = *x,
            Instruction::Write(mem, val) => {
                let bin_value = int_to_bin(*val);
                let mut temp_val = [Bit::Zero;36];
                let mut temp_add = [Bit::Zero;36];
                let bin_mem = int_to_bin(*mem);

                if part == 2 {
                    for (i, (mask_bit, mem_bit)) in mask.iter().zip(bin_mem.iter()).enumerate() {
                        match mask_bit{
                            Bit::Zero=> temp_add[i] = *mem_bit,
                            Bit::One => temp_add[i] = *mask_bit,
                            Bit::X => temp_add[i] = *mask_bit,
                        }
                    }
                }

                for (i, (mask_bit, mem_bit)) in mask.iter().zip(bin_value.iter()).enumerate() {
                    match (mask_bit, mem_bit) {
                        (Bit::Zero, Bit::One) => temp_val[i] = *mask_bit,
                        (Bit::One, Bit::Zero) => temp_val[i] = *mask_bit,
                        _ => temp_val[i] = *mem_bit
                    }
                }

                if part == 1 {
                    memory.insert(*mem, temp_val);
                } else if part == 2{
                    for add in permute(vec![temp_add]).iter() {
                        memory.insert(bin_to_int(*add), bin_value);
                    }
                }
            }
        }
    }
    memory.sum()
}


#[aoc(day14, part1)]
fn solve_part1(input: &Vec<Instruction>) -> usize {
    execute(input, 1)
}

#[aoc(day14, part2)]
fn solve_part2(input: &Vec<Instruction>) -> usize {
   execute(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static INPUT2: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 165);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT2)), 208);
    }



}
