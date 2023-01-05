use itertools::Itertools;
use nom::{
    IResult,
    branch::alt,
    combinator::{opt, map}, 
    character::complete::{line_ending, i32}, 
    sequence:: preceded, 
    bytes::complete::tag, 
    multi::separated_list0,
};

#[derive(Debug, Clone)]
enum Op {
    Add(i32),
    Nothing
}

use Op::*;

fn instruction(s: &'static str) -> IResult<&'static str, Option<i32>> {
    preceded(alt((tag("noop"), tag("addx "))), opt(i32))(s)
}

fn parse(s: &'static str) -> Vec<Op> {
    separated_list0(line_ending, map(instruction, |value| {
        match value {
            Some(value) => Add(value),
            _ => Nothing,
        }
    }))(s).unwrap().1
}

fn simulate(data: Vec<Op>) -> Vec<i32> {
    data.iter().fold((Vec::from([0]), 1), |(mut v, current_value), instruction| {
        match instruction {
            Add(value) => { 
                v.push(current_value);
                v.push(current_value);
                (v, current_value + value)
            },
            Nothing => {
                v.push(current_value);
                (v, current_value)
            }
        }
    }).0
}

fn part1(data: Vec<Op>) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];
    let data = simulate(data);
    data.iter().enumerate().filter(|(i, _)| cycles.contains(i)).map(|(i, v)| (i as i32) * v).sum()
}

fn part2(data: Vec<Op>) -> String {
    let mut strings = Vec::new();
    let data = simulate(data);
    data.iter().skip(1).chunks(40).into_iter()
    .for_each(|v| {
        strings.push("\n");
        v.fold(1, |acc, &e| {
            if acc >= e && acc <= e+2 {
                strings.push("#");
            } else {
                strings.push(".");
            }
            acc + 1
        });
    });
    strings.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(parse(include_str!("test.txt")));
        assert_eq!(result, 13140);
    }

    #[test]
    fn test2() {
        let result = part2(parse(include_str!("test.txt")));
        assert_eq!(result,
        [
        "\n##..##..##..##..##..##..##..##..##..##..",
        "\n###...###...###...###...###...###...###.",
        "\n####....####....####....####....####....",
        "\n#####.....#####.....#####.....#####.....",
        "\n######......######......######......####",
        "\n#######.......#######.......#######....."
        ].join(""));
    }
}

fn main() {
    println!("{}", part1(parse(include_str!("input.txt"))));    
    println!("{}", part2(parse(include_str!("input.txt"))));    
}