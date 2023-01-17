use { Operation::*, DataType::* };
use std::{collections::VecDeque, cmp::Reverse};
use itertools::Itertools;
use nom::{
    IResult,
    combinator::map,
    branch::alt,
    sequence::{preceded, tuple, terminated, separated_pair}, 
    character::complete::{u64, line_ending, anychar, alphanumeric0}, 
    bytes::complete::tag, 
    multi::{separated_list0, fold_many0}
};

#[derive(Debug)]
enum DataType {
    Nothing,
    Items(VecDeque<u64>),
    Operator(Operation),
    Test(u64, u64, u64),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Option<u64>),
    Mul(Option<u64>)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    inspections: u64,
    op: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

type Data = IResult<&'static str, DataType>;

fn items_parser(data: &'static str) -> Data {
    terminated(map(
            separated_list0(tag(", "), u64), 
            |v| Items(VecDeque::from(v))
        ),
        line_ending
    )(data)
}

fn operation_parser(data: &'static str) -> Data {
    terminated(
        map(separated_pair(anychar, tag(" "), 
            map(alt((alphanumeric0, tag("old"))), |s| 
                match s {
                    "old" => None,
                    n => Some(n.parse().unwrap())
        
                }
            )), 
            |(op, n)| {
                match op {
                    '*' => Operator(Mul(n)),
                    _ => Operator(Add(n))
                }
            }
        ), line_ending
    )(data)
}

fn test_parser(data: &'static str) -> Data {
    map(
        tuple((u64, tag("\n    If true: throw to monkey "), u64, tag("\n    If false: throw to monkey "), u64)), 
        |(div_by, _, if_true, _, if_false)| Test(div_by, if_true, if_false)
    )(data)
}

fn parse(data: &'static str) -> Vec<Monkey> {
    separated_list0(tag("\n\n"), fold_many0(alt((
        preceded(tag("Monkey "), map(tuple((u64, tag(":\n"))), |_| Nothing)),
        preceded(tag("  Starting items: "), items_parser), 
        preceded(tag("  Operation: new = old "), operation_parser),
        preceded(tag("  Test: divisible by "), test_parser))),
        || Monkey {
            items: VecDeque::new(),
            inspections: 0,
            op: Mul(None),
            divisible_by: 0,
            if_true: 0,
            if_false: 0,
        },
        |mut monkey, d| {
            match d {
                Items(v) => {
                    monkey.items = v;
                },
                Operator(op) => {
                    monkey.op = op;
                },
                Test(div_by, if_true, if_false) => {
                    monkey.divisible_by = div_by;
                    monkey.if_true = if_true as usize;
                    monkey.if_false = if_false as usize;
                },
                _ => ()
            }
            monkey
        } 
    ))(data).unwrap().1
}

fn solve<const N: usize>(data: &mut [Monkey], f: impl FnOnce(u64) -> u64 + Copy) {
    for _ in 0..N {
        for i in 0..data.len() {
            let mut monkey = data[i].clone();
            if !monkey.items.is_empty() {
                let mut monkey_true = data[monkey.if_true].clone();
                let mut monkey_false = data[monkey.if_false].clone();
                while let Some(item) = monkey.items.pop_front() {
                    monkey.inspections += 1;
                    let worry_level = f(match monkey.op {
                        Mul(n) => if n.is_some() { item * n.unwrap() } else { item * item },
                        Add(n) => if n.is_some() { item + n.unwrap() } else { item + item }
                    });
                    if worry_level % monkey.divisible_by == 0 { 
                        monkey_true.items.push_back(worry_level);
                    } else {
                        monkey_false.items.push_back(worry_level);
                    }
                }
                data[monkey.if_true] = monkey_true;
                data[monkey.if_false] = monkey_false;
                data[i] = monkey;
            }
        }
    }
}

fn monkey_business(data: &Vec<Monkey>) -> u64 {
    data.iter()
        .map(|m| { m.inspections })
        .sorted_by_key(|&n| Reverse(n))
        .take(2)
        .product()
}

fn part1(mut data: Vec<Monkey>) -> u64 {
    solve::<20>(&mut data, |n| n / 3);
    monkey_business(&data)
}

fn part2(mut data: Vec<Monkey>) -> u64 {
    let lcd: u64 = data.iter().map(|m| m.divisible_by).product();
    solve::<10000>(&mut data, |n| n % lcd);
    monkey_business(&data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("test.txt");

    #[test]
    fn test1() {
        let result = part1(parse(&DATA));
        assert_eq!(result, 10605);
    }

    #[test]
    fn test2() {
        let result = part2(parse(&DATA));
        assert_eq!(result, 2713310158);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(parse(input)));
    println!("{}", part2(parse(input)));
}