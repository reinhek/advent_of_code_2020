use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Action> {
    input.lines()
        .map(|l| {
            let action = &l[..1];
            let integer = (&l[1..]).parse().unwrap();

            match action {
                "N" => Action::Move(0, integer),
                "E" => Action::Move(integer, 0),
                "W" => Action::Move(-integer, 0),
                "S" => Action::Move(0, -integer),
                "L" => Action::Rotate(integer as f64),
                "R" => Action::Rotate(-integer as f64),
                "F" => Action::Forward(integer),
                _=> panic!("Error {:?}", action),
            }
        }).collect()
}

struct Ship {
    x: isize,
    y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
    deg: f64,
}

impl Ship {

    fn execute(&mut self, action: Action, part: i32) {
        match action {
            Action::Move(x,y) => {
                match part {
                    1 => {
                        self.x += x;
                        self.y += y;
                    },
                    2 => {
                        self.waypoint_x += x;
                        self.waypoint_y += y;
                    },
                    _=> panic!("Error")
                }
            },
            Action::Rotate(d) => {
                match part {
                    1 => self.deg += d,
                    2 => self.rotate(d),
                    _=> panic!("Error")
                }
            },
            Action::Forward(d) => {
                match part {
                    1 => {
                        self.x += (d as f64 * self.deg.to_radians().cos()) as isize;
                        self.y += (d as f64 * self.deg.to_radians().sin()) as isize;
                    },
                    2 => {
                        self.x += d * self.waypoint_x;
                        self.y += d * self.waypoint_y;
                    },
                    _=> panic!("Error")
                }
            },
        }
    }

    fn rotate(&mut self, degrees: f64) {
        let angle = (self.waypoint_y as f64/self.waypoint_x as f64).atan();
        let mut hyp = 0.0;

        if angle.sin() == 0.0 {
            hyp = self.waypoint_x as f64;
        } else {
            hyp = (self.waypoint_y as f64 / angle.sin());
        }

        self.waypoint_x = ((angle + degrees.to_radians()).cos() * hyp).round() as isize;
        self.waypoint_y = ((angle + degrees.to_radians()).sin() * hyp).round() as isize;
    }

    fn project_course(&mut self, navigation: &Vec<Action>, part: i32) -> isize {
        for action in navigation {
            self.execute(*action, part);
        }
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
    Move(isize, isize),
    Rotate(f64),
    Forward(isize),
}


#[aoc(day12, part1)]
fn solve_part1(input: &Vec<Action>) -> isize {
    let mut ship = Ship { x: 0, y: 0, waypoint_x: 0, waypoint_y: 0, deg: 0.0};
    ship.project_course(input, 1)
}

#[aoc(day12, part2)]
fn solve_part2(input: &Vec<Action>) -> isize {
    let mut ship = Ship { x: 0, y: 0, waypoint_x: 10, waypoint_y: 1, deg: 0.0};
    ship.project_course(input, 2)
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
