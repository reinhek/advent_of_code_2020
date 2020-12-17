use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::collections::HashSet;

type Coordinate = (i32,i32,i32,i32);

#[aoc_generator(day17)]
fn generator(input: &str) -> HashSet<Coordinate>{
    input.lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter(|(_,c)| *c=='#')
                .map(move |(x,_)| (x as i32, y as i32, 0i32, 0i32))
        }).flatten().collect()
}

fn simulate(active_locations: &HashSet<Coordinate>, neighbors: &Vec<Coordinate>) -> usize {
    let mut active_cubes = active_locations.clone();

    for _ in 0..6 {
        let mut for_next: HashSet<Coordinate> = HashSet::new();

        let mut possibilities = active_cubes.clone();
        for cube in active_cubes.iter() {
            for neighbor in neighbors.iter() {
                possibilities.insert((cube.0 + neighbor.0,
                                      cube.1 + neighbor.1,
                                      cube.2 + neighbor.2,
                                      cube.3 + neighbor.3));
            }
        }

        for possibility in possibilities.iter() {
            let is_active = active_cubes.contains(possibility);
            match neighbors.iter().filter(|neighbor| active_cubes.contains(&(possibility.0 + neighbor.0,
                                                                             possibility.1 + neighbor.1,
                                                                             possibility.2 + neighbor.2,
                                                                             possibility.3 + neighbor.3))).count() {
                2 | 3 if is_active => {for_next.insert(possibility.clone()); },
                3 if !is_active => { for_next.insert(possibility.clone()); },
                _=> {}
            }
        }
        active_cubes = for_next;
    }

    active_cubes.len()
}

#[aoc(day17, part1)]
fn solve_part1(input: &HashSet<Coordinate>) -> usize {
    let neighbors: Vec<Coordinate> = iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|x| x.0 != 0 || x.1 != 0 || x.2 != 0)
        .map(|x| (x.0,x.1,x.2,0))
        .collect();

    simulate(input, &neighbors)
}

#[aoc(day17, part2)]
fn solve_part2(input: &HashSet<Coordinate>) -> usize {
    let neighbors: Vec<Coordinate> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|x| x.0 != 0 || x.1 != 0 || x.2 != 0 || x.3 != 0).collect();
    simulate(input, &neighbors)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = ".#.
..#
###";


    #[test]
    fn example1() {
        assert_eq!(solve_part1(&generator(INPUT)), 112);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&generator(INPUT)), 848);
    }


}
