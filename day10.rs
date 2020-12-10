use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input.lines().map(|s| s.parse().unwrap()).collect();
    let max = adapters.iter().max().unwrap().clone();
    adapters.push(0);
    adapters.push(max + 3);
    adapters.sort_unstable();
    adapters
}


fn attach_adapters(adapters: &Vec<usize>) -> usize {
    let mut differences = [0usize; 3];

    for window in adapters.windows(2) {
        differences[window[1] - window[0] - 1] += 1;
    }

    differences[0]*differences[2]
}

#[aoc(day10, part1)]
fn solve_part1(input: &Vec<usize>) -> usize {
    attach_adapters(input)
}

#[aoc(day10, part2)]
fn solve_part2(adapters: &Vec<usize>) -> usize {
    let mut patterns = vec![0usize; adapters.len()];
    patterns[0] = 1;

    for (i, input) in adapters.iter().enumerate() {
        for (j, output) in adapters.iter().enumerate().skip(i+1) {
            if output - input > 3 {
                break;
            }
            patterns[j] += patterns[i];
        }
    }

    *patterns.last().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    static INPUT: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 2310);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT)), 19208);
    }



}
