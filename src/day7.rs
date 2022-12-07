use std::collections::HashMap;

type Input = HashMap<Vec<String>, u32>;

#[aoc_generator(day7)]
pub fn input_generator_mk2(input: &str) -> Input {
    let mut ret: HashMap<Vec<String>, u32> = HashMap::new();

    let mut fs_stack = vec!["/".to_string()];
    input.lines().for_each(|l| {
        let spl = l.split(" ").collect::<Vec<&str>>();
        match l.chars().nth(0).unwrap() {
            '$' => {
                match spl[1] {
                    "cd" => {
                        match spl[2] {
                            ".." => {
                                if fs_stack.len() > 1 {
                                    fs_stack.pop();
                                }
                            },
                            
                            "/" => {
                                fs_stack.drain(..);
                                fs_stack.push("/".to_string());
                            },
                            _ => {
                                fs_stack.push(spl[2].to_string());
                            },
                        };
                    },
                    "ls" => {},
                    _ => todo!(),
                }
            },
            _ => {
                match spl[0] {
                    "dir" => {},
                    _ => {
                        let size = spl[0].parse::<u32>().unwrap();
                        let mut exec_stack = fs_stack.clone();
                        while !exec_stack.is_empty() {
                            ret.entry(exec_stack.clone()).and_modify(|total_size| *total_size += size).or_insert(size);
                            exec_stack.pop();
                        }
                    },
                } 
            },
        }
    });

    ret
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> u32 {
    input.values().filter(|s| {
        *s <= &100000
    }).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> u32 {
    let current = 70000000 - input.get(&vec!["/".to_string()]).unwrap();

    input.values().fold(u32::MAX,|accu, x| {
        if x + current > 30000000 && x < &accu {
            *x
        } else {
            accu
        }
    })
}

