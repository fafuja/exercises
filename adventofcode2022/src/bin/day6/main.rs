#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //let result = part1(include_str!("test.txt"));
        //assert_eq!(result, "CMZ");
    }

    #[test]
    fn test2() {
        //let result = part2(include_str!("test.txt"));
        //assert_eq!(result, "MCD");
    }
}

fn main() {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}