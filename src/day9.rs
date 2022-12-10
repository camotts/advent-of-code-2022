use std::collections::HashSet;

type Input = Vec<(char, u32)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
        input.lines().map(|l| {
            let (left, right) = l.split_once(" ").unwrap();
            (left.chars().next().unwrap(), right.parse::<u32>().unwrap())
        }).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> usize {
    let mut rope = Rope{
        knots: vec![(0,0), (0,0)]
    };

    execute(input, &mut rope)
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> usize {
    let mut rope = Rope{
        knots: vec![(0,0); 10]
    };
    execute(input, &mut rope)
}

fn execute(input: &Input, rope: &mut Rope) -> usize {
    let mut traversed: HashSet<(i32,i32)> = HashSet::new();
    traversed.insert(rope.tail());

    input.iter().for_each(|cmd| {
        match cmd.0 {
            'R' => {
                for _i in 0..cmd.1 {
                    rope.handle_right();
                    traversed.insert(rope.tail());
                }
            },
            'L' => {
                for _i in 0..cmd.1 {
                    rope.handle_left();
                    traversed.insert(rope.tail());
                }
            },
            'U' => {
                for _i in 0..cmd.1 {
                    rope.handle_up();
                    traversed.insert(rope.tail());
                }
            },
            'D' => {
                for _i in 0..cmd.1 {
                    rope.handle_down();
                    traversed.insert(rope.tail());
                }
            },
            _ => todo!(),
        };
    });
    traversed.len()
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<(i32, i32)>
}

impl Rope {
    pub fn handle_right(&mut self) {
        self.knots[0].0 += 1;
        self.follow_up();
    }
    
    pub fn handle_left(&mut self) {
        self.knots[0].0 -= 1;
        self.follow_up();
    }
    
    pub fn handle_up(&mut self) {
        self.knots[0].1 += 1;
        self.follow_up();
    }
    
    pub fn handle_down(&mut self) {
        self.knots[0].1 -= 1;
        self.follow_up();
    }

    pub fn follow_up(&mut self) {
        for i in 1..self.knots.len() {
            if self.knots[i-1].0 == self.knots[i].0 {
                if self.knots[i-1].1.abs_diff(self.knots[i].1) >= 2 {
                    if self.knots[i-1].1 > self.knots[i].1 {
                        self.knots[i].1 += 1
                    } else {
                        self.knots[i].1 -= 1
                    }
                }
                
            }
            
            if self.knots[i-1].1 == self.knots[i].1 {
                if self.knots[i-1].0.abs_diff(self.knots[i].0) >= 2 {
                    if self.knots[i-1].0 > self.knots[i].0 {
                        self.knots[i].0 += 1
                    } else {
                        self.knots[i].0 -= 1
                    }
                }
            }

            if self.knots[i-1].0.abs_diff(self.knots[i].0) >= 2 || self.knots[i-1].1.abs_diff(self.knots[i].1) >= 2 {
                if !self.orthogonal_check(self.knots[i-1], self.knots[i]) {
                    self.knots[i].0 += (self.knots[i-1].0 - self.knots[i].0).signum();
                    self.knots[i].1 += (self.knots[i-1].1 - self.knots[i].1).signum();
                }
            }
        }
    }

    pub fn orthogonal_check(&self, a: (i32, i32), b: (i32, i32)) -> bool {
        a.0 == b.0 || a.1 == b.1
    }

    pub fn tail(&self) -> (i32, i32) {
        return *self.knots.last().unwrap()
    }
}