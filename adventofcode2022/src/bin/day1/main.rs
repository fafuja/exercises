use std::cmp::Reverse;
use itertools::Itertools;

// learning new way of doing it
// from: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day1/main.rs#L10

fn part1(s: &str) -> usize {
    s.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap()
}

fn part2(s: &str) -> usize {
    s.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>())
                          .sorted_unstable_by_key(|&e| Reverse(e))
                          .take(3)
                          .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("test.txt"));
        assert_eq!(result, 24000);
    }

    #[test]
    fn test2() {
        let result = part2(include_str!("test.txt"));
        assert_eq!(result, 45000);
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}