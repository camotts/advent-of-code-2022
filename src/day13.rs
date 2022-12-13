use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self},
    multi::{separated_list1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
    Parser,
};
use std::fmt::Display;
use std::cmp::Ordering::{self, *};


type Input = Vec<Pair>;

#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Clone, Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Display for Packet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Number(l_num), Packet::Number(r_num)) => l_num == r_num,
            (Packet::List(l_vec), Packet::List(r_vec)) => l_vec == r_vec,
            (Packet::List(l_vec), Packet::Number(r_num)) => l_vec == &vec![Packet::Number(*r_num)],
            (Packet::Number(l_num), Packet::List(r_vec)) => &vec![Packet::Number(*l_num)] == r_vec,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(l_num), Packet::Number(r_num)) => l_num.cmp(r_num),
            (Packet::List(l_vec), Packet::List(r_vec)) => l_vec.cmp(r_vec),
            (Packet::List(l_vec), Packet::Number(r_num)) => l_vec.cmp(&vec![Packet::Number(*r_num)]),
            (Packet::Number(l_num), Packet::List(r_vec)) => vec![Packet::Number(*l_num)].cmp(&r_vec),
        }
    }
}
pub fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), packet),
            tag("]"),
        )
        .map(|vec| Packet::List(vec)),
        complete::u32
            .map(|num| Packet::Number(num)),
    ))(input)
}

pub fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, complete::newline, packet).map(
            |(p1, p2)| Pair {
                left: p1,
                right: p2,
            },
        ),
    )(input)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let p = pairs(input);
    p.unwrap().1
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    input.iter().enumerate().filter_map(|(i, pair)| {
       match pair.left.cmp(&pair.right) {
           Less => Some(i+1),
           Equal => todo!(),
           Greater => None,
       }
    }).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> usize {
    let mut input: Vec<Packet> = input.iter().flat_map(|pair| {
        vec![pair.left.clone(), pair.right.clone()]
    }).collect();
    let two = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    input.push(two.clone());
    input.push(six.clone());
    input.sort();
    input.iter().enumerate().filter_map(|(i, item)| {
        if item == &two || item == &six {
            Some(i + 1)
        } else {
            None
        }
    }).product()
}