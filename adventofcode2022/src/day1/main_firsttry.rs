use std::{fs::File, io::Read};

// use "include_str!" macro instead!
fn read_file(input: &str) -> std::io::Result<String> {
    match File::open(input) {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            Ok(s)
        }
        Err(e) => Err(e)
    }
}
#[derive(Debug)]
struct Array {
    node: i32,
    child: Box<Option<Array>>
}

fn main() {
    /* Part 1 
    match read_file("src/input.txt") {
        Ok(s) => print!("{:?} \n", s.split("\n").into_iter().fold((Vec::new(), Vec::new()), |(mut v, mut vs), e| if e.is_empty() {
            v.push(vs);
            (v, Vec::new())
        } else {
            vs.push(e);
            (v, vs)
        }).0.into_iter().fold(0, |acc, e| match e.into_iter().fold(0, |acc, e| acc + e.parse::<i32>().unwrap()) {
            n => if n > acc { n } else { acc }
        }
        )),
        Err(e) => panic!("{}", e.to_string())
    };
    */
    // Part2
    match read_file("src/input.txt") {
        Ok(s) => print!("{:?} \n", s.split("\n").into_iter().fold((Vec::new(), Vec::new()), |(mut v, mut vs), e| if e.is_empty() {
            v.push(vs);
            (v, Vec::new())
        } else {
            vs.push(e);
            (v, vs)
        }).0.into_iter().fold(Vec::from([0,0,0]), |acc, e| match e.into_iter().fold(0, |acc, e| acc + e.parse::<i32>().unwrap()) {
            n => array_to_vec(sort_array(n, vec_to_array(acc.as_slice())), &mut Vec::new()).clone()
        }
        ).into_iter().sum::<i32>()),
        Err(e) => panic!("{}", e.to_string())
    };
}

fn sort_array(n: i32, arr: Option<Array>) -> Option<Array> {
    match arr {
        Some(Array{node, child}) => if n > node { 
            Some(Array { node: n, child: Box::new(sort_array(node, *child)) })
        } else {
            Some(Array{ node, child: Box::new(sort_array(n, *child)) })
        },
        None => None
    }
}

fn array_to_vec(arr: Option<Array>, v: &mut Vec<i32>) -> &Vec<i32> {
    match arr {
        Some(Array{node, child}) => {
            v.push(node);
            array_to_vec(*child, v)
        },
        None => v
    }
}

fn vec_to_array(v: &[i32]) -> Option<Array> {
    match v.first() {
        Some(&n) => Some(Array { node: n, child: Box::new(vec_to_array(&v[1..]))}),
        _ => None
    }
}