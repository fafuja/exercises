use std::{collections::HashSet, hash::Hash};

fn check<T: Hash + Eq>(a: &HashSet<T>, b: &HashSet<T>) -> bool {
    match a.is_superset(b){
        false => b.is_superset(a),
        _ => true,
    }
}

fn into_hashset(s: impl IntoIterator<Item = &'static str>) -> HashSet<usize> {
    let (l, r) = s.into_iter()
     .map(|e| e.parse::<usize>().unwrap())
     .fold((None, None), |(l, r), e|
        if l.is_none() {
            (Some(e), r)
        } else {
            (l, Some(e))
        }
     );
    (l.unwrap()..= r.unwrap()).collect()
}

fn parse(s: impl IntoIterator<Item = &'static str>) -> (HashSet<usize>, bool) {
    s.into_iter()
     .map(|e| (into_hashset(e.split("-")), false))
     .reduce(|(hx, _), (hy, _)| match check(&hx, &hy) {
        true => (hx.intersection(&hy).copied().collect(), true),
        false => (hx.intersection(&hy).copied().collect(), false)
     })
     .expect("shouldnt be empty here")
}

pub fn part1(s: &'static str) -> usize {
    s.lines()
     .map(|e| parse(e.split(",")))
     .fold(0, |acc, e| if e.1 { acc+1 } else { acc })
}

pub fn part2(s: &'static str) -> usize {
    s.lines()
     .map(|e| parse(e.split(",")))
     .fold(0, |acc, e| if e.0.is_empty() { acc } else {acc+1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("test.txt"));
        assert_eq!(result, 2);
    }
    #[test]
    fn test2() {
        let result = part2(include_str!("test.txt"));
        assert_eq!(result, 4);
    }
}

fn main () {
    let result = part1(include_str!("input.txt"));
    println!("{}", result);
    
    let result = part2(include_str!("input.txt"));
    println!("{}", result);
}