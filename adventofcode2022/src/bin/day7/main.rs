use std::collections::HashMap;
use Filesystem::*;
use nom::{
    IResult,
    branch::alt, 
    sequence::{preceded, terminated}, 
    character::streaming::alphanumeric0,
    character::complete::{line_ending, not_line_ending, u32}, 
    bytes::complete::tag, 
    combinator::{map, opt},
    Parser, multi::fold_many0
};

type Parsed = IResult<&'static str, Filesystem>;

type Directories<'a> = HashMap<Vec<&'a str>, usize>;

#[derive(Debug, PartialEq)]
enum Filesystem {
    D(usize),
    C(&'static str)
}

fn parse_directory(s: &'static str) -> Parsed {
    let file = map(u32, |e| e as usize);
    let dir = map(tag("dir"), |_| 0);
    let dirorfile = terminated(alt((file, dir)), not_line_ending.and(opt(line_ending))); //opt(line_ending) return Incomplete error wtf??
    preceded(tag("ls\n"), map(fold_many0(dirorfile, || 0, |acc, e| {
        acc+e
    }), |e| D(e)))(s)
}

fn parse_command(s: &'static str) -> Parsed {
    let cd = map(alt((tag("/"), tag(".."), alphanumeric0)), |e| C(e));
    preceded(tag("cd "), terminated(cd, line_ending))(s)
}

fn parse(s: &'static str) -> Directories {
    let mut v: Vec<&str> = Vec::new();
    fold_many0(
        preceded(tag("$ "), alt((parse_command, parse_directory))), 
        HashMap::new, 
        move |mut h: Directories, e| {
            match e {
                D(n) => {
                    for i in 0..=v.len() {
                        let d = &v[..i];
                        if let Some(s) = h.get_mut(d) {
                            *s += n;
                        } else {
                            h.insert(d.to_vec(), n);
                        }
                    }
                },
                C(c) => { 
                    match c {
                        "/" => v.clear(),
                        ".." => { v.pop(); },
                        s => v.push(s),
                    }
                }
            }
            h
        }
    )(s).unwrap().1
}

fn part1(s: &'static str) -> usize {
    parse(s).values().copied().filter(|&e| e < 100000).sum()
}
fn part2(s: &'static str) -> usize {
    let result = parse(s);
    let dif = 70000000 - result[&vec![]];
    result.values().copied().filter(|&e| e + dif >= 30000000).min().unwrap()
}
//17719346 + 7214296 = 24933642
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("test.txt"));
        assert_eq!(result, 95437);
    }

    #[test]
    fn test2() {
        let result = part2(include_str!("test.txt"));
        println!("{}", result);
        assert_eq!(result, 24933642);
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}