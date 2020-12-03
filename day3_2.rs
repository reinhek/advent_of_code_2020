use aoc_runner_derive::{aoc, aoc_generator};

pub struct Map {
    field: Vec<Vec<bool>>,
    x: usize,
}

impl Map {
    fn get(&self, x_pos: usize, y_pos: usize) -> bool {
        self.field[x_pos][y_pos]
    }
}

#[aoc_generator(day3)]
pub fn day3_generator(input: &str) -> Map {
    let field: Vec<Vec<bool>> = input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();

    let x = field[0].len();

    Map { field, x }
}

pub fn count_trees(inputs: &Map, x_slope: usize, y_slope: usize) -> usize {
    let y = inputs.field.len();
    let mut a = 0;

    (0..y)
        .step_by(y_slope)
        .filter(|&c| {
            let temp = a;
            a += x_slope;
            if a >= inputs.x {
                a -= inputs.x;
            }
            inputs.get(c, temp)
        })
        .count()
}

#[aoc(day3, part1)]
pub fn solve_part1(inputs: &Map) -> usize {
    count_trees(inputs, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(inputs: &Map) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|(x_slope, y_slope)| count_trees(inputs, *x_slope, *y_slope))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = &std::fs::read_to_string("input/test3.txt").unwrap();
        assert_eq!(solve_part1(&day3_generator(input)), 7);
    }

    fn example2() {
        let input: &str = &std::fs::read_to_string("input/test3.txt").unwrap();
        assert_eq!(solve_part2(&day3_generator(input)), 336);
    }
}
