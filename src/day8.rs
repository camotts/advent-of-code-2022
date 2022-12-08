type Input = Vec<Vec<u32>>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    let mut ret = 0;
    for i in 1..input.len()-1 {
        for j in 1..input[0].len()-1 {
            if check_tree(input.to_vec(), i, j) {
                ret += 1;
            }
        }
    }
    ret + (input.len() * 2) + (input[1].len() * 2) - 4
}

pub fn check_tree(map: Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let up = (0..x).rev();
    let down = x+1..map.len();
    let left = (0..y).rev();
    let right = y+1..map[0].len();

    let mut tallest = true;
    for i in up {
        if map[i][y] >= map[x][y] {
            tallest = false;
            break;
        }
    }
    if tallest {
        return true;
    }

    tallest = true;
    for i in down {
        if map[i][y] >= map[x][y] {
            tallest = false;
            break;
        }
    }
    if tallest {
        return true;
    }

    tallest = true;
    for i in left {
        if map[x][i] >= map[x][y] {
            tallest = false;
            break;
        }
    }
    if tallest {
        return true;
    }

    tallest = true;
    for i in right {
        if map[x][i] >= map[x][y] {
            tallest = false;
            break;
        }
    }
    if tallest {
        return true;
    }
    false
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut most_scenic = 0;
    for i in 1..input.len()-1 {
        for j in 1..input[0].len()-1 {
            let new = calc_trees(input.to_vec(), i, j);
            if new > most_scenic {
                most_scenic = new
            }
        }
    }
    most_scenic
}

pub fn calc_trees(map: Vec<Vec<u32>>, x: usize, y: usize) -> u64 {
    let up = (0..x).rev();
    let down = x+1..map.len();
    let left = (0..y).rev();
    let right = y+1..map[0].len();

    let (mut l, mut r, mut u, mut d) = (0, 0, 0, 0);
    for i in up {
        u += 1;
        if map[i][y] >= map[x][y] {
            break;
        }
    }

    for i in down {
        d += 1;
        if map[i][y] >= map[x][y] {
            break;
        }
    }

    for i in left {
        l += 1;
        if map[x][i] >= map[x][y] {
            break;
        }
    }

    for i in right {
        r +=1;
        if map[x][i] >= map[x][y] {
            break;
        }
    }

    l * r * u * d
}