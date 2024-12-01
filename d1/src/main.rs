use std::collections::HashMap;
use std::iter::zip;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() -> std::io::Result<()> {
    println!("Day 01");

    let p1_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p1_result = part1(p1_contents);

    println!("Part 1 result: {p1_result}");

    let p2_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p2_result = part2(p2_contents);

    println!("Part 2 result: {p2_result}");

    return Ok(());
}

fn part1(reader: BufReader<File>) -> i64 {
    let re = Regex::new(r"(?<left>\d+) +(?<right>\d+)").unwrap();

    let mut l: Vec<i64> = vec![];
    let mut r: Vec<i64> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let matches = re.captures(line.as_str()).unwrap();

        l.push(matches["left"].parse().unwrap());
        r.push(matches["right"].parse().unwrap());
    }

    l.sort();
    r.sort();

    return zip(l, r)
        .map(|(l, r)| if r > l { r - l } else { l - r })
        .sum();
}

fn part2(reader: BufReader<File>) -> i64 {
    // capture left & right digits
    let re = Regex::new(r"(?<left>\d+) +(?<right>\d+)").unwrap();

    let mut l: Vec<i64> = vec![];
    let mut r_occurences: HashMap<i64, i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let matches = re.captures(line.as_str()).unwrap();

        l.push(matches["left"].parse().unwrap());

        let r = matches["right"].parse().unwrap();

        r_occurences.entry(r).and_modify(|c| *c += 1).or_insert(1);
    }

    return l
        .iter()
        // get the simularity score for each entry:
        // val * occurences_val
        .map(|val| match r_occurences.get(val) {
            Some(&o) => val * o,
            _ => 0,
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part1(contents);

        assert_eq!(result, 11);
    }

    #[test]
    fn p2() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part2(contents);

        assert_eq!(result, 31);
    }
}
