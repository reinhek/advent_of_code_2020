use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Map {
    field: Vec<Vec<State>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    EmptySeat,
    Occupied,
    Floor,
    Unknown,
}

impl Map {
    fn get_adjacent_count(&self, x: usize, y: usize) -> usize {
        let mut seats = Vec::new();
        if x as i32 -1 >= 0 {
            seats.push(self.field[y][x-1]);
        }
        if x+1 < self.width {
            seats.push(self.field[y][x+1]);
        }
        if y as i32 -1 >= 0{
            seats.push(self.field[y-1][x]);
        }
        if y+1 < self.height {
            seats.push(self.field[y+1][x]);
        }
        if x as i32 -1 >= 0 && y as i32 -1 >= 0 {
            seats.push(self.field[y-1][x-1]);
        }
        if x+1 < self.width && y as i32 -1 >= 0 {
            seats.push(self.field[y-1][x+1]);
        }
        if x as i32 -1 >= 0 && y+1 < self.height {
            seats.push(self.field[y+1][x-1]);
        }
        if x+1 < self.width && y+1 < self.height {
            seats.push(self.field[y+1][x+1]);
        }

        seats.iter().filter(|&&s| s == State::Occupied).count()
    }

}

#[aoc_generator(day11)]
fn generator(input: &str) -> Map {
    let field: Vec<Vec<State>> = input
        .lines()
        .map(|s| s.chars().map(|c| match c {
            'L' => State::EmptySeat,
            '.' => State::Floor,
            _=> State::Unknown,
        }).collect())
        .collect();

    let width = field[0].len();
    let height = field.len();

    Map { field, width, height }
}

fn simulate(map: &Map) -> usize {
    let mut has_changed = true;
    let mut occupied = 0;
    let mut map_to_check = map.clone();
    let mut map_to_change = map.clone();

    while has_changed {
        has_changed = false;
        for (y, line) in map_to_check.field.iter().enumerate() {
            for (x, state) in line.iter().enumerate() {
                match state {
                    State::EmptySeat => {
                        if map_to_check.get_adjacent_count(x, y) == 0{
                            map_to_change.field[y][x] = State::Occupied;
                            has_changed = true;
                            occupied += 1;
                        }
                    },
                    State::Occupied => {
                        if map_to_check.get_adjacent_count(x, y) >= 4{
                            map_to_change.field[y][x] = State::EmptySeat;
                            has_changed = true;
                            occupied -= 1;
                        }
                    },
                    _ => continue
                };
            }
        }
        map_to_check = map_to_change.clone();
    }

    occupied
}

#[aoc(day11, part1)]
fn solve_part1(input: &Map) -> usize {
    simulate(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    static INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 37);
    }




}
