use std::cmp::Reverse;
use itertools::Itertools;
pub fn part1and2(n: usize) -> usize {
    // learning new way of doing it
    // from: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day1/main.rs#L10
    let text = include_str!("input.txt");
    //println!("{:?}", text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap())

    // Part 1
    if n == 1 {
        text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap()
    } else {
    // Part 2
        text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>())
                          .sorted_unstable_by_key(|&e| Reverse(e))
                          .take(3)
                          .sum::<usize>()
    }
}