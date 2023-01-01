use std::collections::HashSet;
use nom::{
    branch::alt,
    combinator::{value, opt},
    multi::fold_many1,
    bytes::complete::tag, 
    character::complete::i32, 
    sequence::{separated_pair, terminated}, 
    character::complete::line_ending,
    IResult
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(i32, i32);

type Points = Vec<(HashSet<Point>, Option<Point>)>;

fn direction(s: &'static str) -> IResult<&'static str, Point> {
    alt((
        value(Point(1,0), tag("R")), 
        value(Point(-1,0), tag("L")), 
        value(Point(0,1), tag("U")), 
        value(Point(0,-1), tag("D")))
    )(s)
}

fn parse(s: &'static str) -> Vec<Point> {
    fold_many1(
        terminated(separated_pair(direction, tag(" "), i32), opt(line_ending)), 
        || Vec::from([Point(0,0)]),
        |mut v, (dir, amount)| {
            v.last().copied().and_then(|lhp| { 
                for i in 1..= amount{
                    let p = add(&lhp,&mult(&dir, &i));
                    v.push(p);
                }
                Some(v)
            }).unwrap()
        }
    )(s).unwrap().1
}

fn mult(dir: &Point, n: &i32) -> Point {
    Point(dir.0 * n, dir.1 * n)
}

fn add(a: &Point, b: &Point) -> Point {
    Point(b.0 + a.0, b.1 + a.1)
}

fn calculate_position(current: &Point, last: &Point) -> Point {
    let diff = (current.0 - last.0, current.1 - last.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        Point(last.0 + diff.0.signum(), last.1 + diff.1.signum())
    } else {
        last.clone()
    }
}

fn simulate<const N: usize>(data: Vec<Point>) -> Points {
    data.into_iter().fold(vec![(HashSet::new(), None); N-1], |knots, mut current_point| {
        knots.into_iter().map(|(mut points, last_tail_position)| {
            match last_tail_position {
                Some(point) => {
                    current_point = calculate_position(&current_point, &point);
                    points.insert(current_point);
                    (points, Some(current_point))
                },
                None => {
                    points.insert(current_point);
                    (points, Some(current_point))
                }
            }
        }).collect()
    })
}

fn part1(data: Vec<Point>) -> usize {
    simulate::<2>(data).first().unwrap().0.len()
}

fn part2(data: Vec<Point>) -> usize {
    simulate::<10>(data).last().unwrap().0.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(parse(include_str!("test.txt")));
        assert_eq!(result, 13);
    }

    #[test]
    fn test2() {
        let result = part2(parse(include_str!("test2.txt")));
        assert_eq!(result, 36);
    }
}

fn main() {
    println!("{}", part1(parse(include_str!("input.txt"))));    
    println!("{}", part2(parse(include_str!("input.txt"))));    
}