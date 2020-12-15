use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;



#[aoc_generator(day15)]
fn generator(input: &str) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for (i,s) in input.split(',').enumerate() {
        map.insert(s.parse().unwrap(), i+1);
    }
    map
}

fn find_nth(mut numbers: HashMap<usize, usize>, n: usize) -> usize {
    let mut number = 0;

    for turn in numbers.len()+1..n {
        if numbers.contains_key(&number) {
            let index = numbers.get(&number).unwrap().to_owned();
            numbers.insert(number, turn);
            number = turn - index;
        } else {
            numbers.insert(number, turn);
            number = 0
        }
    }
    number
}

#[aoc(day15, part1)]
fn solve_part1(input: &HashMap<usize, usize>) -> usize {
    find_nth(input.clone(), 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &HashMap<usize, usize>) -> usize {
    find_nth(input.clone(), 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "0,3,6";


    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 436);
    }



}
