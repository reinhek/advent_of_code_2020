use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
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
    fn get(&self, x: usize, y:usize) -> Option<&State> {
        self.field.get(x).and_then(|r|r.get(y))
    }

    fn count_visible(&self, x: usize, y: usize, max_range: usize) -> usize {
        let mut count = 0;

        let slopes = vec![(1,1),
                              (1,0),
                              (1,-1),
                              (0,1),
                              (0,-1),
                              (-1,1),
                              (-1,0),
                              (-1,-1)];

        for dir in 0..slopes.len() {
            let mut rd = y as i32;
            let mut cd = x as i32;

            for _ in 0..max_range {
                rd += slopes[dir].0;
                cd += slopes[dir].1;

                match self.get(rd as usize, cd as usize) {
                    Some(State::Occupied) => {
                        count += 1;
                        break;
                    },
                    Some(State::Floor) => {
                        continue;
                    },
                    _=> {
                        break;
                    }
                }

            }
        }


        count
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
    let mut occupied = 0;
    let mut map_to_check = map.clone();
    let mut map_to_change = map.clone();

    loop {
        for (y, line) in map_to_check.field.iter().enumerate() {
            for (x, state) in line.iter().enumerate() {
                match state {
                    State::EmptySeat => {
                        if map_to_check.count_visible(x,y, 1) == 0{
                            map_to_change.field[y][x] = State::Occupied;
                            occupied += 1;
                        }
                    },
                    State::Occupied => {
                        if map_to_check.count_visible(x, y, 1) >= 4{
                            map_to_change.field[y][x] = State::EmptySeat;
                            occupied -= 1;
                        }
                    },
                    _ => continue
                };
            }
        }
        if map_to_check == map_to_change {
            break;
        }
        map_to_check = map_to_change.clone();
    }

    occupied
}

fn simulate_2(map: &Map) -> usize {
    let mut occupied = 0;
    let mut map_to_check = map.clone();
    let mut map_to_change = map.clone();

    loop {
        for (y, line) in map_to_check.field.iter().enumerate() {
            for (x, state) in line.iter().enumerate() {
                match state {
                    State::EmptySeat => {
                        if map_to_check.count_visible(x,y, std::usize::MAX) == 0{
                            map_to_change.field[y][x] = State::Occupied;
                            occupied += 1;
                        }
                    },
                    State::Occupied => {
                        if map_to_check.count_visible(x, y, std::usize::MAX) >= 5{
                            map_to_change.field[y][x] = State::EmptySeat;
                            occupied -= 1;
                        }
                    },
                    _ => continue
                };
            }
        }
        if map_to_check == map_to_change {
            break;
        }
        map_to_check = map_to_change.clone();
    }

    occupied
}

#[aoc(day11, part1)]
fn solve_part1(input: &Map) -> usize {
    simulate(input)
}

#[aoc(day11, part2)]
fn solve_part2(input: &Map) -> usize {
    simulate_2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT)), 26);
    }




}
