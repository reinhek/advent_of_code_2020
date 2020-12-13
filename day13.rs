use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
struct Schedule {
    timestamp: usize,
    bus: Vec<usize>,
    delay: Vec<usize>,
}

impl Schedule {
    fn find_earliest(&self) -> (usize,usize) {
        let sched: Vec<usize> = self.bus.iter().map(|&b| {
            b - (self.timestamp % b) + self.timestamp
        }).collect();
        let earliest = sched.iter().min().unwrap();
        let index = sched.iter().position(|b| b == earliest).unwrap();

        (*earliest, self.bus[index])
    }

    fn find_earliest_with_offset(&self) -> usize {
        let mut result = 0;
        let mut prod = 1;

        for (remainder, modulo) in self.delay.iter().zip(&self.bus) {
            while (result + *remainder) % *modulo != 0 {
                result += prod;
            }
            prod *= *modulo;
        }
        result
    }
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Schedule {
    let mut iter = input.lines();
    let timestamp = iter.next().unwrap().parse().unwrap();
    let mut bus = Vec::new();
    let mut delay = Vec::new();
    for (i, item) in iter.next().unwrap().split(',').enumerate() {
        if item != "x" {
            bus.push(item.parse().unwrap());
            delay.push(i);
        }
    }
    Schedule {timestamp, bus, delay}
}

#[aoc(day13, part1)]
fn solve_part1(input: &Schedule) -> usize {
    let (sched, bus) = input.find_earliest();
    (sched - input.timestamp) * bus
}

#[aoc(day13, part2)]
fn solve_part2(input: &Schedule) -> usize {
    input.find_earliest_with_offset()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "939
7,13,x,x,59,x,31,19";

    static INPUT2: &'static str = "939
17,x,13,19";

    static INPUT3: &'static str = "939
1789,37,47,1889";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 295);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT)), 1068781);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part2(&generator(INPUT3)), 1202161486);
    }



}
