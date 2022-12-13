use itertools::Itertools;
use nom::{
    IResult,
    bytes::streaming::take,
    sequence::{terminated, tuple},
    multi::{many_till, separated_list0},
    bytes::streaming::tag,
    combinator::{opt, eof, map},
    character::complete::line_ending,
    character::streaming::u32
};
use Crate::*;

type Parsed = IResult<&'static str, (Vec<&'static str>, &'static str)>;
type Stacks = Vec<Vec<Crate>>;
type Operations = Vec<Operation>;

#[derive(Debug)]
struct Operation {
    quantity: usize,
    from: usize,
    to: usize
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Crate {
    Value(char),
    Empty
}

fn convert(s: &str) -> Crate {
    let e = s.chars().nth(1).unwrap();
    if !e.is_whitespace() {
        Value(e)
    } else {
        Empty
    }
}

fn exec_instruction(op: Operation, s: &mut Stacks, mut f: impl FnMut(Vec<Crate>) -> Vec<Crate>) -> Stacks {
    let mut v: Vec<Crate> = Vec::new();
    for _ in 0..op.quantity {
        let vc = s.get_mut(op.from).unwrap();
        let c = vc.pop().unwrap();
        v.push(c);
    }
    s.get_mut(op.to).unwrap().append(f(v).as_mut());
    s.clone()
}

fn organize_stacks(s: Stacks, c: usize) -> Stacks {
    let mut v: Stacks = vec![Vec::new(); c];
    for e in s {
        for (i, &c) in e.iter().enumerate() {
            if c == Empty {
                continue;
            } else {
                v.get_mut(i).and_then(|v| Some(v.push(c)));
            }
        }
    }
    v
}

fn parse_stack(s: &'static str) -> Parsed {
    let identifier = || terminated(take(3usize), opt(tag(" ")));
    many_till(identifier(), tag("\n"))(s)
}

fn parse_stacks(s: &'static str) -> Stacks {
    let (_, (s, _)) = many_till(parse_stack, eof)(s).unwrap();
    let v: Stacks = s.into_iter()
    .map(|(e, _)| e.into_iter().map(|e| convert(e)).collect())
    .rev()
    .skip(2)
    .collect();
    let c = v.first().unwrap().len();
    organize_stacks(v, c)
}

fn parse_operation(s: &'static str) -> IResult<&'static str, Operation> {
    let line_parsed = tuple((
        tag("move "), u32, tag(" from "), u32, tag(" to "), u32
    ));
    map(line_parsed, |e| Operation {
        quantity: e.1 as usize,
        from: (e.3 as usize)-1,
        to: (e.5 as usize)-1
    })(s)
}

fn parse_operations(s: &'static str) -> Operations {
    let (_, lines) = separated_list0(line_ending , parse_operation)(s).unwrap();
    lines
}

fn part1(s: &'static str) -> String {
    let strs: (&str, &str) = s.split_inclusive("\n\n").collect_tuple().expect("2 elements");
    let ops = parse_operations(strs.1);
    let out = ops.into_iter()
    .fold(parse_stacks(strs.0), |v, e| exec_instruction(e, &mut v.clone(), |v| v));
    
    out.iter().fold(Vec::new(), |mut v, e| {
        let e = match e.last().unwrap() {
            Value(e) => e,
            _ => &' '
        };
        v.push(e);
        v
    }).iter().join("")
}

fn part2(s: &'static str) -> String {
    let strs: (&str, &str) = s.split_inclusive("\n\n").collect_tuple().expect("2 elements");
    let ops = parse_operations(strs.1);
    let out = ops.into_iter()
    .fold(parse_stacks(strs.0), |v, e| exec_instruction(e, &mut v.clone(), |v| v.into_iter().rev().collect()));
    
    out.iter().fold(Vec::new(), |mut v, e| {
        let e = match e.last().unwrap() {
            Value(e) => e,
            _ => &' '
        };
        v.push(e);
        v
    }).iter().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("test.txt"));
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test2() {
        let result = part2(include_str!("test.txt"));
        assert_eq!(result, "MCD");
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}