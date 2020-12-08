use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let instruction = iter.next().unwrap();
        let integer = iter.next().unwrap().parse().unwrap();

        match instruction {
            "nop" => Instruction::Nop(integer),
            "acc" => Instruction::Acc(integer),
            "jmp" => Instruction::Jmp(integer),
            _=> Instruction::Unknown,
        }
    }).collect()
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
    Unknown,
}


pub enum ExitMode {
    InfiniteLoop,
    EndOfProgram,
}

pub fn return_acc(code: &Vec<Instruction>) -> (i32,ExitMode) {
    let mut acc = 0;
    let mut map = vec![false; code.len()];
    let mut iter: usize = 0;

    loop {
        if map[iter] {
            return (acc, ExitMode::InfiniteLoop)
        }
        map[iter] = true;
        match code[iter] {
            Instruction::Acc(x) => {
                acc += x;
                iter += 1;
            },
            Instruction::Jmp(x) => {
                iter = (iter as i32 + x) as usize;
            },
            Instruction::Nop(_) => {
                iter += 1;
            },
            _ => {}
        }
        if iter >= code.len(){
            return (acc, ExitMode::EndOfProgram)
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(code: &Vec<Instruction>) -> i32 {
    return_acc(code).0
}

#[aoc(day8, part2)]
pub fn solve_part2(code: &Vec<Instruction>) -> i32 {
    let mut codes = code.clone();

    (0..code.len()).find_map(|i| {
        let old_instruction = codes[i];
        codes[i] = match codes[i] {
            Instruction::Jmp(x) => Instruction::Nop(x),
            Instruction::Nop(x) => Instruction::Jmp(x),
            _=> {return  None}
        };

        match return_acc(&codes) {
            (_, ExitMode::InfiniteLoop) =>  {
                codes[i] = old_instruction;
                None
            },
            (acc, ExitMode::EndOfProgram) => Some(acc)
        }

    }).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(generator(INPUT).as_mut()), 8);
    }


}
