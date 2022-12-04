use std::collections::HashSet;
use itertools::Itertools;

fn parse(c: char) -> usize {
    if c.is_lowercase() { 
        (c as usize) - 96 
    } else {
        (c as usize) - 38
    }
}

fn intersect_item(group: impl IntoIterator<Item = &'static str>) -> usize {
    group
        .into_iter()
        .map(|items| items.chars().map(|e| parse(e)).collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .unwrap()
        .into_iter()
        .exactly_one()
        .expect("only one usize element here")
}

pub fn part1(s: &'static str) -> usize {
    s.lines()
    .map(|l| l.split_at(l.len()/2))
    .map(|(l, r)| intersect_item([l,r]))
    .sum()
}

pub fn part2(s: &'static str) -> usize {
    s.lines()
    .chunks(3)
    .into_iter()
    .map(intersect_item)
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result: usize= part1(include_str!("test.txt"));
        assert_eq!(result, 157);
    }
    #[test]
    fn test2() {
        let result = part2(include_str!("test.txt"));
        assert_eq!(result, 70);
    }
}