use std::collections::HashMap;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

type Parsed = HashMap<Coordinate, u32>;
type Coordinate = (i32, i32);

fn parse(s: &'static str) -> Parsed {
    s.split("\n").enumerate().flat_map(|(y, e)| {
        e.chars().enumerate().map(|(x, e)| {
            ((x as i32, y as i32), e.to_digit(10).unwrap())
        }).collect::<Vec<(Coordinate, u32)>>()
    }).collect()
}

fn add(a: &Coordinate, b: &Coordinate) -> Coordinate {
    (b.0 + a.0, b.1 + a.1)
}

fn line(data: &Parsed, a: &Coordinate, d: & Coordinate) -> Vec<u32> {
    let mut v = Vec::new();
    let mut temp = data.get_key_value(&add(a, d));
    while temp != None {
        v.push(temp.unwrap().1.clone());
        temp = data.get_key_value(&add(temp.unwrap().0, d)); 
    }
    v
}

fn part1(p: &Parsed) -> usize {
    let data = p.iter();
    data.fold(0, |acc, (a, va)| {
        let dir = [(1,0),(0,1),(-1,0),(0,-1)].map(|d| line(p, a, &d));
        if dir.iter().any(|v| v.iter().all(|vb| va > vb)) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(p: &Parsed) -> usize {
    let data = p.iter();
    data.fold(0, |acc, (a, va)| {
        let dir = [(1,0),(0,1),(-1,0),(0,-1)].map(|d| line(p, a, &d));
        let scenic: usize = dir.map(|v| {
            match v.iter().fold_while(0, |i, vb| if vb >= va { Done(i + 1) } else { Continue(i + 1) }) {
                Done(n) => n,
                Continue(n) => n
            }
        }).iter().product();
        if scenic > acc { scenic } else { acc }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse(include_str!("test.txt")));
        assert_eq!(result, 21);
    }

    #[test]
    fn test2() {
        let result = part2(&parse(include_str!("test.txt")));
        assert_eq!(result, 8);
    }
}

fn main() {
    println!("{}", part1(&parse(include_str!("input.txt"))));    
    println!("{}", part2(&parse(include_str!("input.txt"))));    
}