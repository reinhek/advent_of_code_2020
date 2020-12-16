use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

#[aoc_generator(day16)]
fn generator(input: &str) -> Ticket {
    lazy_static! {
        static ref REG_RANGE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    }
    let sections : Vec<&str>= input.split("\n\n").collect();

    let rules = sections[0].lines().map(|line| {
        Rules { ranges : REG_RANGE.captures_iter(line)
            .map(|cap| ((&cap[1]).parse().unwrap(),(&cap[2]).parse().unwrap()))
            .collect::<Vec<(u32,u32)>>() }
    }).collect_vec();

    let yours = sections[1].lines().skip(1).next().unwrap().split(',')
        .map(|c| c.parse().unwrap()).collect();

    let nearby = sections[2].lines().skip(1).map(|line| {
        line.split(',').map(|c| c.parse().unwrap()).collect()
    }).collect();

    Ticket {yours, nearby, rules}
}

#[derive(Debug, Clone)]
struct Ticket {
    yours: Vec<u32>,
    nearby: Vec<Vec<u32>>,
    rules: Vec<Rules>,
}

#[derive(Debug, Clone)]
struct Rules {
    ranges: Vec<(u32,u32)>
}

impl Rules {
    fn in_range(&self, num: u32) -> bool {
        (num >= self.ranges[0].0 && num <= self.ranges[0].1) ||
            (num >= self.ranges[1].0 && num <= self.ranges[1].1)
    }
}
fn get_set_bit(num: u32) -> Option<u32> {
    (0..=32).find(|x| num & (1 << x) != 0)
}


#[aoc(day16, part1)]
fn solve_part1(input: &Ticket) -> u32 {
    input.nearby.iter()
        .flatten()
        .filter(|&&num| input.rules.iter().all(|r| !r.in_range(num)))
        .sum()
}

#[aoc(day16, part2)]
fn solve_part2(input: &Ticket) -> usize {
    let valid_tickets = input.nearby.iter()
        .filter(|ticket|
            ticket.iter().all(|&num|
                input.rules.iter().any(|r|
                    r.in_range(num))))
        .collect_vec();

    let mut possibilities: Vec<u32> = input.rules.iter()
        .map(|r| {
            let mut possibility_index = 0;
            for index in 0..input.rules.len() {
                if valid_tickets.iter().all(|ticket| r.in_range(ticket[index])) {
                    possibility_index |= 1 << index;
                }
            }
            possibility_index
        }).collect_vec();

    let mut departures = vec![0; input.rules.len()];
    while let Some(&x) = possibilities.iter().find(|&possibility| possibility.count_ones() == 1) {
        for (index, possibility) in possibilities.iter_mut().enumerate() {
            if *possibility == x {
                departures[index] = get_set_bit(x).unwrap()
            }
            *possibility &= !x;
        }
    }

    departures[0..6].iter().map(|&field| input.yours[field as usize] as usize).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    static INPUT2: &'static str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";


    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 71);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT2)), 12*11*13);
    }
    

}
