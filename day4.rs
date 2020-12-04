use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use regex::Regex;
use lazy_static::lazy_static;

pub struct Passport<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn parse(data: &'a str) -> Self {
        let data = data
            .split_whitespace()
            .map(|f| {
                let mut iter = f.split(':');
                let field = iter.next().unwrap();
                let value = iter.next().unwrap();
                (field,value)
            })
            .collect();

        Self{data}
    }

    fn is_valid(&self) -> bool {
        let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_fields
            .iter()
            .all(|field| self.data.contains_key(field))
    }

    fn is_valid_2(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let in_range = |range: RangeInclusive<u32>, value: Option<&&str>| -> bool {
            value.and_then(|val| val.parse::<u32>().ok())
                .filter(|num| range.contains(num))
                .is_some()
        };

        lazy_static! {
            static ref REG_HGT: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$").unwrap();
            static ref REG_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref REG_ECL: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref REG_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        let eval_regex = move |regex: &Regex, value: Option<&&str>| -> bool {
            value.filter(|val| regex.is_match(val)).is_some()
        };

        let eval_height = |value: Option<&&str>| -> bool {
            value.and_then(|val| REG_HGT.captures(val))
                .and_then(|cap| {
                    Some((
                            cap.name("value")?.as_str().parse::<u32>().ok()?,
                            cap.name("unit")?.as_str(),
                    ))
                }).filter(|(val, unit)| match *unit {
                    "cm" => (150..=193).contains((val)),
                    "in" => (59..=76).contains(val),
                    _=> false,
            }).is_some()
        };

        in_range(1920..=2002, self.data.get("byr"))
            && in_range(2010..=2020, self.data.get("iyr"))
            && in_range(2020..=2030, self.data.get(("eyr")))
            && eval_height(self.data.get("hgt"))
            && eval_regex(&REG_HCL, self.data.get("hcl"))
            && eval_regex(&REG_ECL, self.data.get("ecl"))
            && eval_regex(&REG_PID, self.data.get("pid"))

    }

}

pub fn parse_batch(input: &str) -> impl Iterator<Item = Passport> {
   input.split("\n\n").map(|split_input| Passport::parse(split_input))
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    parse_batch(input).filter(|passport|passport.is_valid()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    parse_batch(input).filter(|passport|passport.is_valid_2()).count()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = &std::fs::read_to_string("input/test4.txt").unwrap();
        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn example2() {
        let input: &str = &std::fs::read_to_string("input/test4_2.txt").unwrap();
        assert_eq!(solve_part2(input), 4);
    }


}
