use std::collections::HashSet;

#[aoc_generator(day3, part1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| {
        let (a, b) = l.split_at(l.chars().count()/2);
        let aHash = a.chars().collect::<HashSet<char>>();
        let bHash = b.chars().collect::<HashSet<char>>();
        let res = aHash.intersection(&bHash).next().unwrap();
        let mut cc = *res as u32;
        if res.is_uppercase() {
            cc = cc + 58;
        }
        let aa = 'a' as u32;
        let ret =  cc - aa + 1;
        ret
    }).collect::<Vec<u32>>()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_part2(input: &str) -> Vec<u32> {
    input.split("\n").collect::<Vec<&str>>().chunks(3).map(|chunk| {
        let mut chunkVec = chunk.iter().flat_map(|item| {
            vec![item.chars().collect::<HashSet<char>>()]
        }).collect::<Vec<HashSet<char>>>();
        let mut inter = chunkVec[0].clone();

        let folded = chunkVec.iter().fold(inter, |accum, item| {
            accum.clone().intersection(&item.clone()).map(|c| *c ).collect::<HashSet<char>>()
        });

        let res = folded.iter().next().unwrap();
        let mut cc = *res as u32;
        if res.is_uppercase() {
            cc = cc + 58;
        }
        let aa = 'a' as u32;
        let ret =  cc - aa + 1;
        ret
    }).collect::<Vec<u32>>()
}


#[aoc(day3, part1)]
pub fn part1(input: &Vec<u32>) -> u32 {
    input.iter().sum()
}


#[aoc(day3, part2)]
pub fn part2(input: &Vec<u32>) -> u32 {
    input.iter().sum()
}