use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Input {
    Containers: Vec<Vec<char>>,
    Instructions: Vec<String>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut ret = Input{
        Containers: Vec::new(),
        Instructions: instructions.lines().map(|l| l.to_string()).collect::<Vec<String>>(),
    };

    let mut revCrates = crates.split("\n").collect::<Vec<&str>>();
    revCrates.reverse();
    
    for i in 0..revCrates[1].split(" ").collect::<Vec<&str>>().len() {
        ret.Containers.push(Vec::new());
    }

    for layer in revCrates.iter().skip(1) {
        let mut ct = 0;
        for ch in layer.chars().skip(1).step_by(4) {
            if ch != ' ' {
                ret.Containers[ct].push(ch);
            }
            ct = ct + 1;
        }
    }
    ret
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let mut clone = input.clone();
    for l in clone.Instructions.iter() {
        let sp = l.split(" ").collect::<Vec<&str>>();
        for i in 0..sp[1].parse::<u32>().unwrap() {
            let val = clone.Containers[sp[3].parse::<usize>().unwrap()-1].pop().unwrap();
            clone.Containers[sp[5].parse::<usize>().unwrap()-1].push(val);
        }
    }
    clone.Containers.iter().map(|s| {
        s.last().unwrap()
    }).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let mut clone = input.clone();
    for l in clone.Instructions.iter() {
        let sp = l.split(" ").collect::<Vec<&str>>();
        let mut tmp = Vec::new();
        for i in 0..sp[1].parse::<u32>().unwrap() {
            let val = clone.Containers[sp[3].parse::<usize>().unwrap()-1].pop().unwrap();
            tmp.push(val);
        }
        for i in 0..sp[1].parse::<u32>().unwrap() {
            let val = tmp.pop().unwrap();
            clone.Containers[sp[5].parse::<usize>().unwrap()-1].push(val);
        }
    }
    clone.Containers.iter().map(|s| {
        s.last().unwrap()
    }).collect()
}
