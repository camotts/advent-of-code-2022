use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> BinaryHeap<u32> {
    input.split("\n\n").map(|s| {
        s.lines().map(|l| {
            l.parse::<u32>().unwrap()
        }).sum::<u32>()
    }).collect::<BinaryHeap<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &BinaryHeap<u32>) -> u32 {
    let mut cln = input.clone();
    cln.pop().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &BinaryHeap<u32>) -> u32 {
    let cln = input.clone();
    return cln.into_iter_sorted().take(3).sum();
}