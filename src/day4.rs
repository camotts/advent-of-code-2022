use lazy_static::lazy_static;
use regex::Regex;

type Input = Vec<((u32, u32), (u32, u32))>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\d*)-(\\d*),(\\d*)-(\\d*)$").unwrap();
    }
    input.lines().map(|l| {
        let cap = RE.captures_iter(l).next().unwrap();
        let a1 = cap[1].parse::<u32>().unwrap();
        let a2 = cap[2].parse::<u32>().unwrap();
        let a3 = cap[3].parse::<u32>().unwrap();
        let a4 = cap[4].parse::<u32>().unwrap();
        if a1 < a3 {
            ((a1, a2 ), (a3, a4))
        } else {
            ((a3, a4), (a1, a2))
        }
    }).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> u32 {
    input.iter().map(|pair| {
        if pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1 || pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1 {
            1
        } else {
            0
        }
    }).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> u32 {
    input.iter().map(|pair| {
        if !(pair.0.1 < pair.1.0) {
            1
        } else {
            0
        }
    }).sum()
}