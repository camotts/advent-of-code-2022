use std::collections::HashMap;
use std::collections::VecDeque;

type Input = Vec<Vec<Tile>>;

#[derive(PartialEq, Debug)]
pub enum Tile {
    Start,
    End,
    Tile(char),
}

impl Tile {
    pub const fn value(&self) -> u32 {
        match *self {
            Tile::Start => 0,
            Tile::End => 25,
            Tile::Tile(x) => x as u32 - b'a' as u32,
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
        input.lines().map(|l| l.chars().map(|c|{
            match c {
                'S' => Tile::Start,
                'E' => Tile::End,
                _ => Tile::Tile(c)
            }
        }).collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut traversed = HashMap::new();
    let start = input.iter().enumerate().find_map(|(i, lines)| {
        lines.iter().enumerate().find_map(|(j, t)| {
            if *t == Tile::Start {
                Some((i,j))
            } else {
                None
            }
        })
    }).unwrap();

    let end = input.iter().enumerate().find_map(|(i, lines)| {
        lines.iter().enumerate().find_map(|(j, t)| {
            if *t == Tile::End {
                Some((i,j))
            } else {
                None
            }
        })
    }).unwrap();

    let mut queue: VecDeque<(usize, usize)> = vec![start].into();
    let mut ret = 0;
    while let Some(curr) = queue.pop_front() {
        if input[curr.0][curr.1] == Tile::End {
            break
        }
        let mut adjacents = vec![];
        if curr.0 > 0 {
            if !traversed.contains_key(&(curr.0-1, curr.1)) {
                adjacents.push((curr.0-1, curr.1))
            }
        }
        if curr.0 < input.len() - 1 {
            if !traversed.contains_key(&(curr.0+1, curr.1)) {
                adjacents.push((curr.0+1, curr.1))
            }
        }
        if curr.1 > 0 {
            if !traversed.contains_key(&(curr.0, curr.1-1)) {
                adjacents.push((curr.0, curr.1-1))
            }
        }
        if curr.1 < input[0].len() - 1 {
            if !traversed.contains_key(&(curr.0, curr.1+1)) {
                adjacents.push((curr.0, curr.1+1))
            }
        }
        adjacents.iter().filter(|(x,y)|{
            input[curr.0][curr.1].value() + 1 >= input[*x][*y].value()
        }).for_each(|pt| {
            traversed.insert(*pt, curr);
            queue.push_back(*pt)
        });
    }

    let mut path = end;
    while let Some(prev) = traversed.get(&path) {
        ret += 1;

        if *prev == start {
            break;
        }
        path = *prev;
    }

    ret
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> u32 {
    
    let mut starts = input
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(j, v)| if v.value() == 0 { Some((i, j)) } else { None })
    })
    .collect::<Vec<(usize, usize)>>();

    let end = input.iter().enumerate().find_map(|(i, lines)| {
        lines.iter().enumerate().find_map(|(j, t)| {
            if *t == Tile::End {
                Some((i,j))
            } else {
                None
            }
        })
    }).unwrap();

    let mut min = u32::MAX;

    while let Some(start) = starts.pop() {
        let mut traversed: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = vec![start].into();
        let mut ret = 0;
        while let Some(curr) = queue.pop_front() {
            if input[curr.0][curr.1] == Tile::End {
                break
            }
            let mut adjacents = vec![];
            if curr.0 > 0 {
                if !traversed.contains_key(&(curr.0-1, curr.1))
                    && input[curr.0][curr.1].value() + 1 >= input[curr.0-1][curr.1].value() {
                    adjacents.push((curr.0-1, curr.1));
                    traversed.insert((curr.0-1, curr.1), curr);
                    queue.push_back((curr.0-1, curr.1))
                }
            }
            if curr.0 < input.len() - 1 {
                if !traversed.contains_key(&(curr.0+1, curr.1))
                && input[curr.0][curr.1].value() + 1 >= input[curr.0+1][curr.1].value() {
                    adjacents.push((curr.0+1, curr.1));
                    traversed.insert((curr.0+1, curr.1), curr);
                    queue.push_back((curr.0+1, curr.1))
                }
            }
            if curr.1 > 0 {
                if !traversed.contains_key(&(curr.0, curr.1-1))
                && input[curr.0][curr.1].value() + 1 >= input[curr.0][curr.1-1].value() {
                    adjacents.push((curr.0, curr.1-1));
                    traversed.insert((curr.0, curr.1-1), curr);
                    queue.push_back((curr.0, curr.1-1))
                }
            }
            if curr.1 < input[0].len() - 1 {
                if !traversed.contains_key(&(curr.0, curr.1+1))
                && input[curr.0][curr.1].value() + 1 >= input[curr.0][curr.1+1].value() {
                    adjacents.push((curr.0, curr.1+1));
                    traversed.insert((curr.0, curr.1+1), curr);
                    queue.push_back((curr.0, curr.1+1))
                }
            }
        }

        let mut path = end;

        if !traversed.contains_key(&path) {
            continue;
        }
        while let Some(prev) = traversed.get(&path) {
            ret += 1;

            if *prev == start {
                break;
            }
            path = *prev;
        }
        if ret < min {
            min = ret
        }
    }

    min
}
