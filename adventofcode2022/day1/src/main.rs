use std::cmp::Reverse;
use itertools::Itertools;
fn main() {
    // learning new way of doing it
    // from: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day1/main.rs#L10
    let text = include_str!("input.txt");
    //println!("{:?}", text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap())

    //part1
    //println!("{:?}", text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap())

    //part2
    println!("{:?}", text.split("\n\n").map(|e| e.lines().map(|e| e.parse::<usize>().unwrap()).sum::<usize>())
                                       .sorted_unstable_by_key(|&e| Reverse(e))
                                       .take(3)
                                       .sum::<usize>())
}