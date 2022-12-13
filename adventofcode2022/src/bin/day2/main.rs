use std::collections::HashMap;
pub fn part1(s: &str) -> Vec<usize>{
       let hm = HashMap::from([ 
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
       ]);
    s.lines().map(|e| hm.get(e).unwrap().clone()).collect()
}
pub fn part2(s: &str) -> Vec<usize>{
    let hm = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);
    s.lines().map(|e| hm.get(e).unwrap().clone()).collect()
}

pub fn calculate(v: Vec<usize>) -> usize {
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = part1(include_str!("test.txt"));
        assert_eq!(15, calculate(input));
    }
    #[test]
    fn test2() {
        let input = part2(include_str!("test.txt"));
        assert_eq!(12, calculate(input));
    }
}

fn main (){
    let result = part1(include_str!("input.txt"));
    println!("{}", calculate(result));
    let result = part2(include_str!("input.txt"));
    println!("{}", calculate(result));
}