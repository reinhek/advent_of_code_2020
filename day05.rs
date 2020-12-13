use aoc_runner_derive::{aoc, aoc_generator};

pub fn convert_to_ids(input: &str) -> impl Iterator<Item = isize> + '_ {
    input.lines().map(|line| {
        let bin_str = line.chars().map(|c| {
            if c == 'F' || c == 'L' {
                '0'
            }
            else {
                '1'
            }
        }).collect::<String>();

        isize::from_str_radix(&bin_str, 2).unwrap()
    })
}

pub fn find_seat(seats: Vec<isize>) -> isize {
    for (i, num) in seats.iter().enumerate() {
        if seats[i+1] - *num != 1 {
            return num + 1
        }
    }
    return 1
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> isize {
    convert_to_ids(input).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> isize {
    let mut seats: Vec<isize> = convert_to_ids(input).collect();
    seats.sort();
    find_seat(seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        assert_eq!(solve_part1(input), 820);
    }


}
