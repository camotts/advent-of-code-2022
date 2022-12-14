use std::collections::HashMap;

type Input = HashMap<(usize, usize), char>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let mut cave = HashMap::new();
    input.lines().for_each(|l| {
        let items = l.split(" -> ");
        let walls: Vec<(usize, usize)> = items.map(|i| {
            let (l, r) = i.split_once(",").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        }).collect();
        for idx in 0..walls.len() - 1 {
            let (i_start,i_end) = if walls[idx].0 > walls[idx+1].0 {
                (walls[idx+1].0,walls[idx].0)
            } else {
                (walls[idx].0,walls[idx+1].0)
            };
            let (j_start,j_end) = if walls[idx].1 > walls[idx+1].1 {
                (walls[idx+1].1,walls[idx].1)
            } else {
                (walls[idx].1,walls[idx+1].1)
            };
            for j in j_start..=j_end {
                for i in i_start..=i_end {
                    cave.insert((i,j), '▉');
                }
            }
        }
    });
    cave
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> i64 {
    let mut cave = input.clone();
    let start = (500,0);
    cave.insert((500, 0), 'ஃ');
    let lowest_point = cave.keys().map(|k| k.1).max().unwrap();
    let leftmost = cave.keys().map(|k| k.0).min().unwrap();
    let rightmost = cave.keys().map(|k| k.0).max().unwrap();
    let mut grains = 0;
    loop {
        let mut new = start.clone();
        while let Some(next) = next_move(new, &cave) {
            new = next;
            if new.1 > lowest_point + 2 {
                print(&cave, lowest_point, leftmost-(lowest_point/2), rightmost+(lowest_point/2));
                return grains
            }
        }
        grains += 1;
        cave.insert(new, 'ஃ');
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut cave = input.clone();
    let start = (500,0);
    cave.insert((500, 0), 'ஃ');
    let lowest_point = cave.keys().map(|k| k.1).max().unwrap();
    let leftmost = cave.keys().map(|k| k.0).min().unwrap();
    let rightmost = cave.keys().map(|k| k.0).max().unwrap();
    for i in 300..800 {
        cave.insert((i, lowest_point+2), '▉');
    }
    let mut grains = 0;
    loop {
        let mut new = start.clone();
        while let Some(next) = next_move(new, &cave) {
            new = next;
        }
        grains += 1;
        if new == start {
            print(&cave, lowest_point, leftmost-(lowest_point/2), rightmost+(lowest_point/2));
            return grains
        }
        cave.insert(new, 'ஃ');
    }
    0
}

pub fn print(cave: &HashMap<(usize, usize), char>, lowest: usize, leftmost: usize, rightmost: usize) {
    for j in 0..=lowest+3 {
        for i in leftmost-1..=rightmost+1 {
            match cave.get(&(i,j)) {
                Some(c) => print!("{}", c),
                _ => print!(" "),
            }
        }
        print!("\n");
    }
}

pub fn next_move(point: (usize, usize), cave: &HashMap<(usize, usize), char>) -> Option<(usize, usize)> {
    if !cave.contains_key(&(point.0, point.1+1)) {
        Some((point.0, point.1+1))
    } else if !cave.contains_key(&(point.0-1, point.1+1)) {
        Some((point.0-1, point.1+1))
    } else if !cave.contains_key(&(point.0+1, point.1+1)) {
        Some((point.0+1, point.1+1))
    } else {
        None
    }
}