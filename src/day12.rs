use std::collections::HashSet;
use std::collections::HashMap;

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
            Tile::Tile(x) => x as u32 - 97,
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
    let mut traversed = HashSet::new();
    //traversed.insert((0,0));
    //traversed.insert((1,0));
    //print(input, &mut traversed);
    //0
    //let dist = traverse(input, &mut traversed, (20,0));
    let dist = traverse(input, &mut traversed, (0,0));
    //print(input, &traversed);
    dist
}

#[derive(Clone, Debug)]
pub struct Node {
    pos: (usize, usize),
    parent: Option<Box<Node>>,
}

pub fn traverse(grid: &Vec<Vec<Tile>>, traversed: &mut HashSet<(usize, usize)>, current: (usize, usize)) -> u32 {
    traversed.insert(current);
    let mut queue: Vec<Node> = vec![Node{
        pos: current,
        parent: None,
    }];
    let mut ret = 0;
    while !queue.is_empty() {
        //print(grid, &traversed);
        let curr = queue.pop().unwrap();
        println!("{:?}", grid[curr.pos.0][curr.pos.1]);
        if grid[curr.pos.0][curr.pos.1] == Tile::End {
            let mut test = HashMap::new();
            let mut prev = Some(Box::new(curr.clone()));
            let mut tmp = curr.parent;
            while tmp.is_some() {
                ret += 1;
                println!("{:?}", tmp.clone().unwrap().pos);

                let mut ch = ' ';
                if tmp.clone().unwrap().pos.0 > prev.clone().unwrap().pos.0 {
                    ch = '^';
                }
                if tmp.clone().unwrap().pos.0 < prev.clone().unwrap().pos.0 {
                    ch = 'v';
                }
                if tmp.clone().unwrap().pos.1 < prev.clone().unwrap().pos.1 {
                    ch = '>';
                }
                if tmp.clone().unwrap().pos.1 > prev.clone().unwrap().pos.1 {
                    ch = '<';
                }

                test.insert(tmp.clone().unwrap().pos, ch);
                tmp = tmp.clone().unwrap().parent;
                prev = prev.clone().unwrap().parent;
            }
            print(grid, &test, false);
            //println!("{:?}", curr);
            return ret
        }
        let mut adjacents = vec![];
        if curr.pos.0 > 0 {
            if !traversed.contains(&(curr.pos.0-1, curr.pos.1)) {
                adjacents.push((curr.pos.0-1, curr.pos.1))
            }
        }
        if curr.pos.0 < grid.len() - 1 {
            if !traversed.contains(&(curr.pos.0+1, curr.pos.1)) {
            adjacents.push((curr.pos.0+1, curr.pos.1))
            }
        }
        if curr.pos.1 > 0 {
            if !traversed.contains(&(curr.pos.0, curr.pos.1-1)) {
            adjacents.push((curr.pos.0, curr.pos.1-1))
            }
        }
        if curr.pos.1 < grid[0].len() - 1 {
            if !traversed.contains(&(curr.pos.0, curr.pos.1+1)) {
            adjacents.push((curr.pos.0, curr.pos.1+1))
            }
        }
        adjacents.iter().filter(|(x,y)|{
            println!("({},{}): {:?} vs ({},{}): {:?} is  {} > {}: {}", curr.pos.0, curr.pos.1,grid[curr.pos.0][curr.pos.1], x, y, grid[*x][*y], grid[curr.pos.0][curr.pos.1].value() + 1 , grid[*x][*y].value(), grid[curr.pos.0][curr.pos.1].value() + 1 >= grid[*x][*y].value());
            grid[curr.pos.0][curr.pos.1].value() + 1 >= grid[*x][*y].value()
        }).for_each(|pt| {
            //println!("Attempting to traverse to ({},{})", pt.0, pt.1);
            //let mut line = String::new();
            //std::io::stdin().read_line(&mut line);
            traversed.insert(*pt);
            queue.push(Node{
                pos: pt.clone(),
                parent: Some(Box::new(curr.clone())),
            })
        });
    }
    ret
}

pub fn traverse_dfs(grid: &Vec<Vec<Tile>>, traversed: &mut HashSet<(usize, usize)>, current: (usize, usize)) -> u32 {
    traversed.insert(current);
    //println!("({:?},{:?}): {:?}", current.0, current.1, grid[current.0][current.1]);
    //print(grid, &traversed);
    if grid[current.0][current.1] == Tile::End {
        return 0
    }
    let mut adjacents = vec![];
    if current.0 > 0 {
        if !traversed.contains(&(current.0-1, current.1)) {
            adjacents.push((current.0-1, current.1))
        }
    }
    if current.0 < grid.len() - 1 {
        if !traversed.contains(&(current.0+1, current.1)) {
        adjacents.push((current.0+1, current.1))
        }
    }
    if current.1 > 0 {
        if !traversed.contains(&(current.0, current.1-1)) {
        adjacents.push((current.0, current.1-1))
        }
    }
    if current.1 < grid[0].len() - 1 {
        if !traversed.contains(&(current.0, current.1+1)) {
        adjacents.push((current.0, current.1+1))
        }
    }

    let trav: Vec<u32> = adjacents.iter().filter(|(x, y)| {
        //println!("({},{}): {:?} vs ({},{}): {:?} is  {} > {}: {}", current.0, current.1,grid[current.0][current.1], x, y, grid[*x][*y], grid[current.0][current.1].value() + 1 , grid[*x][*y].value(), grid[current.0][current.1].value() + 1 >= grid[*x][*y].value());
        grid[current.0][current.1].value() + 1 >= grid[*x][*y].value()
     }).map(|(x,y)| {
        //println!("Attempting to traverse to ({},{})", x, y);
        //let mut line = String::new();
        //std::io::stdin().read_line(&mut line);
        traverse(grid, &mut traversed.clone(), (*x,*y))
    }).collect();
    //println!("{:?}", adjacents);
    //println!("Traversal result: {:?}", trav);
    match trav.iter().min() {
        Some(x) => x + 1,
        _ => u32::MAX - 100,
    }
}

pub fn print(grid: &Vec<Vec<Tile>>, traversed: &HashMap<(usize, usize), char>, print_grid: bool) {
    println!("\n\n");
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let mut sym = ' ';
            match grid[x][y] {
                Tile::Start => sym = 'S',
                Tile::End => sym = 'E',
                Tile::Tile(x) => {
                    sym = '.';
                    if print_grid {
                        sym = x;
                    }
                },
            }
            if traversed.contains_key(&(x,y)) {
                sym = *traversed.get(&(x,y)).unwrap();
            }

            print!("{}", sym);
        }
        println!("");
    }
    println!("\n\n");
}