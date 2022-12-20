

fn part1(s: &'static str) -> usize {
    unimplemented!()
}
fn _part2(s: &'static str) -> usize {
    unimplemented!()
}
//17719346 + 7214296 = 24933642
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("test.txt"));
        assert_eq!(result, 21);
    }

    #[test]
    fn test2() {
        //let result = part2(include_str!("test.txt"));
        //assert_eq!(result, 24933642);
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
}