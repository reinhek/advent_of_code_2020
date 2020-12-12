use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Action> {
    input.lines()
        .map(|l| {
            let action = &l[..1];
            let integer = (&l[1..]).parse().unwrap();

            match action {
                "N" => Action::North(integer),
                "E" => Action::East(integer),
                "W" => Action::West(integer),
                "S" => Action::South(integer),
                "L" => Action::Left(integer as f64),
                "R" => Action::Right(integer as f64),
                "F" => Action::Forward(integer),
                _=> panic!("Error {:?}", action),
            }
        }).collect()
}

struct Ship {
    x: isize,
    y: isize,
    deg: f64,
}

impl Ship {
    fn rotate(&mut self, degrees: f64) {
        let angle = (self.y as f64/self.x as f64).atan();
        let mut hyp = 0.0;

        if angle.sin() == 0.0 {
            hyp = self.x as f64;
        } else {
            hyp = (self.y as f64 / angle.sin());
        }

        self.x = ((angle + degrees.to_radians()).cos() * hyp).round() as isize;
        self.y = ((angle + degrees.to_radians()).sin() * hyp).round() as isize;
    }

    fn project_course(&mut self, navigation: &Vec<Action>) -> isize {
        for action in navigation {
            execute(self, *action);
        }
        self.x.abs() + self.y.abs()
    }

    fn project_course_2(&mut self, navigation: &Vec<Action>, waypoint: &mut Ship) -> isize {
        for action in navigation {
            match action {
                Action::Forward(x) => {
                    self.x += (*x as isize) * waypoint.x;
                    self.y += (*x as isize) * waypoint.y;
                },
                Action::Left(x) => {
                    waypoint.rotate(*x);
                },
                Action::Right(x) => {
                    waypoint.rotate(-*x);
                },
                _=> {
                    execute(waypoint, *action);
                }
            }

        }
        self.x.abs() + self.y.abs()
    }
}

fn execute(object: &mut Ship, action: Action) {
    match action {
        Action::North(d) => {
            object.y += d as isize;
        },
        Action::East(d) => {
            object.x += d as isize;
        },
        Action::West(d) => {
            object.x -= d as isize;
        },
        Action::South(d) => {
            object.y -= d as isize;
        },
        Action::Left(d) => {
            object.deg += d;
        },
        Action::Right(d) => {
            object.deg -= d;
        },
        Action::Forward(d) => {
            object.x += (d as f64 * object.deg.to_radians().cos()) as isize;
            object.y += (d as f64 * object.deg.to_radians().sin()) as isize;
        },
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(f64),
    Right(f64),
    Forward(i32),
}


#[aoc(day12, part1)]
fn solve_part1(input: &Vec<Action>) -> isize {
    let mut ship = Ship { x: 0, y: 0, deg: 0.0};
    ship.project_course(input)
}

#[aoc(day12, part2)]
fn solve_part2(input: &Vec<Action>) -> isize {
    let mut ship = Ship { x: 0, y: 0, deg: 0.0};
    let mut waypoint = Ship {x: 10, y: 1, deg: 0.0};
    ship.project_course_2(input, &mut waypoint)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT)), 286);
    }


}
