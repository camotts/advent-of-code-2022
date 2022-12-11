type Input = Vec<String>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
        input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut cycle = Vec::new();
    let mut last = 1;
    cycle.push(last);
    input.iter().for_each(|l| {
        match l.chars().next().unwrap() {
            'n' => cycle.push(last),
            _ => {
                let (_, right) = l.split_once(" ").unwrap();
                cycle.push(last);
                last += right.parse::<i64>().unwrap();
                cycle.push(last);
            },
        }
    });
    20 * cycle.iter().nth(20-1).unwrap() +
    60 * cycle.iter().nth(60-1).unwrap() +
    100 * cycle.iter().nth(100-1).unwrap() +
    140 * cycle.iter().nth(140-1).unwrap() +
    180 * cycle.iter().nth(180-1).unwrap() +
    220 * cycle.iter().nth(220-1).unwrap()
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> String {
    let mut cycle = Vec::new();
    let mut last = 1;
    let mut ret: String = "\n".to_string();
    cycle.push(last);
    input.iter().for_each(|l| {
        match l.chars().next().unwrap() {
            'n' => cycle.push(last),
            _ => {
                let (_, right) = l.split_once(" ").unwrap();
                cycle.push(last);
                last += right.parse::<i64>().unwrap();
                cycle.push(last);
            },
        }
    });
    for i in 0..6 {
        for j in 0..40 {
            let curr = *cycle.iter().nth((i*40)+j).unwrap();
            if curr == j as i64 || curr == j as i64 + 1 || curr == j as i64 - 1 {
                ret += &"#".to_string();
            } else {
                ret += &" ".to_string();
            }
        }
        ret += &"\n".to_string();
    };
    ret
}