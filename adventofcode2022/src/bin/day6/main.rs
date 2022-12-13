use itertools::Itertools;

fn solve(s: & str, n: usize) -> usize {
    s.as_bytes()
    .windows(n)
    .position(|e| e.iter().all_unique())
    .unwrap() + n
}
fn part1(s: &'static str) -> usize {
    solve(s, 4)
}
fn part2(s: &'static str) -> usize {
    solve(s, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 7);
        let result = part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 5);
        let result = part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 6);
        let result = part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 10);
        let result = part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 11);
    }

    #[test]
    fn test2() {
        let result = part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 19);
        let result = part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 23);
        let result = part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 23);
        let result = part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 29);
        let result = part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 26);
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}