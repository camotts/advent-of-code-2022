use std::collections::HashSet;

type Input = Vec<char>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
        input.chars().collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..input.len()-4 {
        set.insert(input[i]);
        set.insert(input[i+1]);
        set.insert(input[i+2]);
        set.insert(input[i+3]);
        if set.len() == 4 {
            return i + 4;
        }
        set.drain();
    }
    0
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..input.len()-14 {
        let mut max = i+14;
        if max > input.len() {
            max = input.len();
        }
        for j in i..max {
            set.insert(input[j]);
        }
        if set.len() == 14 {
            return i + 14;
        }
        set.drain();
    }
    0
}