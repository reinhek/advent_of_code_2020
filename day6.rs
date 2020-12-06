use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use itertools::Itertools;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|x| String::from(x)).collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &[String]) -> usize {
    let mut group_counts: Vec<usize> = vec![];
    let mut group_map: HashSet<char> = HashSet::with_capacity(26);
    for group in input {
        for answer in group.lines() {
            for question in answer.chars() {
                group_map.insert(question);
            }
        }
        group_counts.push(group_map.len());
        group_map.clear();
    }
    group_counts.iter().sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &[String]) -> usize {
    let mut group_counts: Vec<usize> = vec![];
    for group in input {
        let mut group_maps: Vec<HashSet<char>> = vec![];
        for answer in group.lines() {
            let mut hs: HashSet<char> = HashSet::with_capacity(answer.len());
            for question in answer.chars() {
                hs.insert(question);
            }
            group_maps.push(hs)
        }
        group_counts.push(
            group_maps
                .iter()
                .cloned()
                .fold1(|acc, x| &acc & &x)
                .unwrap()
                .len(),
        );
    }
    group_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(generator(INPUT).as_ref()), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(generator(INPUT).as_ref()), 6);
    }


}
