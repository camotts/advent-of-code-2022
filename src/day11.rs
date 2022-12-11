use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list1,
    sequence::{preceded},
    IResult,
};

type Input = Vec<Monkey>;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_target: usize,
    false_target: usize,
    inspected: u64,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Square,
    Add(u64),
    Mult(u64),
}

fn parse_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, Operation::Square))
}

fn parse_mult(input: &str) -> IResult<&str, Operation> {
    let (input, n) = preceded(
        tag("* "),
        complete::u64,
    )(input)?;
    Ok((input, Operation::Mult(n)))
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
    let (input, n) = preceded(
        tag("+ "),
        complete::u64,
    )(input)?;
    Ok((input, Operation::Add(n)))
}


fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _n) = preceded(tag("Monkey "), complete::u64)(input)?;
    

    let (input, numbers) = preceded(
        tag(":\n  Starting items: "),
        separated_list1(tag(", "),complete::u64),
    )(input)?;

    let (input, operation) = preceded(
        tag("\n  Operation: new = old "),
        alt((parse_mult, parse_add, parse_square)),
    )(input)?;

    let (input, division) = preceded(
        tag("\n  Test: divisible by "), complete::u64
    )(input)?;

    let (input, true_target) = 
    preceded(
        tag("\n    If true: throw to monkey "), complete::u64
    )(input)?;

    let (input, false_target) = 
    preceded(
        tag("\n    If false: throw to monkey "), complete::u64
    )(input)?;

    Ok((input, Monkey{
        items: numbers,
        operation: operation,
        divisor: division,
        true_target: true_target as usize,
        false_target: false_target as usize,
        inspected: 0,
    }))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, output) = separated_list1(tag("\n\n"), parse_monkey)(input)?;
    Ok((input, output))
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let p = parse_input(input);
    p.unwrap().1
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> u64 {
    execute(&mut input.clone(), 3, 20)
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> u64 {
    execute(&mut input.clone(), 1, 10000)
}

pub fn execute(input: &mut Input, worry_div: u64, iterations: usize) -> u64 {
    let common_div: u64 = input.iter().map(|m| m.divisor).product();
    for _ in 0..iterations {
        for i in 0..input.len() {
            for item_idx in 0..input[i].items.len() {
                let item = input[i].items[item_idx];
                input[i].inspected += 1;
                let new = match input[i].operation {
                    Operation::Add(n) => {
                        ((item + n) / worry_div) % common_div
                    },
                    Operation::Mult(n) => {
                        ((item * n) / worry_div) % common_div
                    },
                    _ => {
                        ((item * item) / worry_div) % common_div
                    },
                };
                
                let target = if new % input[i].divisor == 0 {
                    input[i].true_target
                } else {
                    input[i].false_target
                };
                input[target].items.push(new);
            }
            input[i].items.clear();
        }
    }
    input.sort_unstable_by_key(|k| k.inspected);
    input.iter().rev().take(2).map(|m| { m.inspected }).product()
}