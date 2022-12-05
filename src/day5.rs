#[derive(Debug, Clone)]
pub struct Input {
    containers: Vec<Vec<char>>,
    instructions: Vec<String>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut ret = Input{
        containers: Vec::new(),
        instructions: instructions.lines().map(|l| l.to_string()).collect::<Vec<String>>(),
    };

    let mut rev_crates = crates.split("\n").collect::<Vec<&str>>();
    rev_crates.reverse();
    
    for _ in 0..rev_crates[1].split(" ").collect::<Vec<&str>>().len() {
        ret.containers.push(Vec::new());
    }

    for layer in rev_crates.iter().skip(1) {
        let mut ct = 0;
        for ch in layer.chars().skip(1).step_by(4) {
            if ch != ' ' {
                ret.containers[ct].push(ch);
            }
            ct = ct + 1;
        }
    }
    ret
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let mut clone = input.clone();
    for l in clone.instructions.iter() {
        let sp = l.split(" ").collect::<Vec<&str>>();
        for _ in 0..sp[1].parse::<u32>().unwrap() {
            let val = clone.containers[sp[3].parse::<usize>().unwrap()-1].pop().unwrap();
            clone.containers[sp[5].parse::<usize>().unwrap()-1].push(val);
        }
    }
    clone.containers.iter().map(|s| {
        s.last().unwrap()
    }).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let mut clone = input.clone();
    for l in clone.instructions.iter() {
        let sp = l.split(" ").collect::<Vec<&str>>();
        let mut tmp = Vec::new();
        for _ in 0..sp[1].parse::<u32>().unwrap() {
            let val = clone.containers[sp[3].parse::<usize>().unwrap()-1].pop().unwrap();
            tmp.push(val);
        }
        for _ in 0..sp[1].parse::<u32>().unwrap() {
            let val = tmp.pop().unwrap();
            clone.containers[sp[5].parse::<usize>().unwrap()-1].push(val);
        }
    }
    clone.containers.iter().map(|s| {
        s.last().unwrap()
    }).collect()
}
