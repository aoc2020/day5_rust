use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = read_input();
    let mut seatnums : Vec<u64> = input.iter().map(|s| to_id(s)).collect();
    seatnums.sort();
    let first = seatnums.first().unwrap();
    let last = seatnums.last().unwrap();
    let taken_seats : HashSet<&u64> = HashSet::from_iter(seatnums.iter());
    let all_seats = (first.clone()..last.clone());
    let empty_seats: Vec<u64> = all_seats.into_iter().filter(|seat| !taken_seats.contains(seat)).collect();
    println!("Answer 1: {}",last);
    println!("Answer 2: {}",empty_seats[0]);
}

fn char_to_digit(c: char) -> u64 {
    if c == 'B' || c == 'R' {
        1
    } else {
        0
    }
}

fn to_id(s: &String) -> u64 {
    s.chars().map(|c| char_to_digit(c)).fold(0, |num,digit| num*2 + digit)
}

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = vec!();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(s) = line {
                // println!("Processing: {}", s);
                input.push(s);
            }
        }
    }
    return input;
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}