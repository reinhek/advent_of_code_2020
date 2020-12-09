use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn pair_exists(set: Vec<usize>, number: usize) -> bool {
    for (i, num1) in set.iter().enumerate() {
        for num2 in set.iter().skip(i) {
            if *num1 + *num2 == number && *num1 != *num2 {
                return true
            }
        }
    }
    false
}

fn find_invalid(numbers: &Vec<usize>, preamble: usize) -> usize {
    let mut first_index = 0;
    let mut number_index = preamble;
    while number_index < numbers.len() {
        let preamble_set = numbers[first_index..number_index].to_vec();
        let number = numbers[number_index];

        if !pair_exists(preamble_set, number) {
           return number;
        }

        first_index += 1;
        number_index += 1;
    }

    0
}

fn find_weakness(numbers: &Vec<usize>, invalid: usize) -> usize {
    for (i, num1) in numbers.iter().enumerate() {
        let set = &numbers[i..];
        let mut sum = 0;
        let mut num2 = 0;

        for num in set.iter() {
            if *num > num2 {
                num2 = *num;
            }

            sum += *num;

            if sum == invalid {
                return num1 + num2
            }
        }
    }
    0
}


#[aoc(day9, part1)]
fn solve_part1(input: &Vec<usize>) -> usize {
    find_invalid(input, 25)
}

#[aoc(day9, part2)]
fn solve_part2(input: &Vec<usize>) -> usize {
    let invalid = find_invalid(input, 25);
    find_weakness(input, invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";



    #[test]
    fn example1() {
        assert_eq!(find_invalid(&generator(INPUT), 5), 127);
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("input/test/test9.txt").unwrap();
        assert_eq!(find_invalid(&generator(input.as_ref()), 25), 49);
    }

    #[test]
    fn example3() {
        let numbers = &generator(INPUT);
        let invalid = find_invalid( numbers, 5);
        assert_eq!(find_weakness(numbers, invalid), 62);
    }


}
