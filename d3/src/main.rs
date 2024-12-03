use std::{
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

fn main() -> std::io::Result<()> {
    println!("Day 03");

    let p1_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p1_result = part1(p1_contents);

    println!("Part 1 result: {p1_result} safe reports");

    return Ok(());
}

fn part1(mut reader: BufReader<File>) -> i32 {
    let re = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();

    let mut calculations = String::new();
    reader.read_to_string(&mut calculations).unwrap();

    return re
        .captures_iter(&calculations)
        .map(|m| {
            let l: i32 = m["l"].parse().unwrap();
            let r: i32 = m["r"].parse().unwrap();

            return l * r;
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn p1() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part1(contents);

        assert_eq!(result, 161);
    }
}
