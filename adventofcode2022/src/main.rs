mod day1;
mod day3;
mod day4;
fn main () {

    println!("      Day 1     ");
    let result = day1::part1and2(1);
    println!("{}", result);
    let result = day1::part1and2(2);
    println!("{}", result);

    println!("      Day 3     ");
    let result = day3::part1(include_str!("day3/input.txt"));
    println!("{}", result);
    let result = day3::part2(include_str!("day3/input.txt"));
    println!("{}", result);

    println!("      Day 4     ");
    let result = day4::part1(include_str!("day4/input.txt"));
    println!("{}", result);    
    let result = day4::part2(include_str!("day4/input.txt"));
    println!("{}", result);
}