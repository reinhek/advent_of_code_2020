use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn read_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day3, part1)]
fn solve_part1(data: &Vec<String>) -> i32 {
    let mut result = 0;
    let mut pos: usize = 0;
    for (i, line) in data.iter().enumerate() {
        let mut row: Vec<char> = line.chars().collect();
        if pos >= row.len() {
            pos -= row.len();
        }
        row.rotate_left(pos);

        for (j, row_char) in row.iter().enumerate() {
            let character = row_char.to_owned();
            if (character == '#' && i != 0) && j == 0{
                result += 1;
                continue;
            }
            if pos % 3 == 0 && j != 0{
                break;
            }
        }
        pos += 3;
    }

    return result
}

#[aoc(day3, part2)]
fn solve_part2(data: &Vec<String>) -> usize {
    let mut result = vec![0;5];
    let slopes = vec![[1,1], [3,1], [5,1], [7,1], [1, 2]];

    for (a, slope) in slopes.iter().enumerate() {
        let mut x_pos: usize = 0;
        let mut y_pos: usize = 0;
        let x = slope[0];
        let y = slope[1];
        for (i, line) in data.iter().enumerate() {
            let mut row: Vec<char> = line.chars().collect();

            if x_pos >= row.len() {
                x_pos -= row.len();
            }

            if y_pos % y != 0{
                y_pos += 1;
                continue;
            }

            row.rotate_left(x_pos);

            for (j, row_char) in row.iter().enumerate() {
                let character = row_char.to_owned();
                if (character == '#' && i != 0) && j == 0 {
                    result[a] += 1;
                    continue;
                }
                if x_pos % x == 0 && j != 0 {
                    break;
                }
            }
            x_pos += x;
            y_pos += 1;
            }
        }
    result.iter().fold(1usize, |a, &b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

        let read_file = std::fs::read_to_string("input/test3_2.txt").unwrap();
        let input2 = read_file.lines().map(|s| s.to_owned()).collect();

        assert_eq!(solve_part1(&input2), 2);
    }

    #[test]
    fn example2() {
        let read_file = std::fs::read_to_string("input/test3.txt").unwrap();
        let input2 = read_file.lines().map(|s| s.to_owned()).collect();

        assert_eq!(solve_part2(&input2), 336);
    }

}
