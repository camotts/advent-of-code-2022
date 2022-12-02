use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| {
        s.chars().filter(|c| *c != ' ').collect()
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> u32 {
    let points =  HashMap::from([
        ('A', HashMap::from([
            ('X', 4),
            ('Y', 8),
            ('Z', 3)
        ])),
        ('B', HashMap::from([
            ('X', 1),
            ('Y', 5),
            ('Z', 9)
        ])),
        ('C', HashMap::from([
            ('X', 7),
            ('Y', 2),
            ('Z', 6)
        ]))
    ]);
    input.iter().map(|p| {
       calculate(p[0], p[1], &points)
    }).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> u32 {
    let points =  HashMap::from([
        ('A', HashMap::from([
            ('X', 3),
            ('Y', 4),
            ('Z', 8)
        ])),
        ('B', HashMap::from([
            ('X', 1),
            ('Y', 5),
            ('Z', 9)
        ])),
        ('C', HashMap::from([
            ('X', 2),
            ('Y', 6),
            ('Z', 7)
        ]))
    ]);
    input.iter().map(|p| {
        calculate(p[0], p[1], &points)
    }).sum()
}

pub fn calculate(opp: char, player: char, points: &HashMap<char, HashMap<char, u32>>) -> u32 {
    match points.get(&opp) {
        Some(play) => match play.get(&player) {
                Some(val) => *val,
                _ => 0
        },
        _ =>0
    }
}