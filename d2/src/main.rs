use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    println!("Day 02");

    let p1_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p1_result = part1(p1_contents);

    println!("Part 1 result: {p1_result} safe reports");

    return Ok(());
}

fn part1(reader: BufReader<File>) -> usize {
    let mut statuses: Vec<Status> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let current: Vec<i32> = line.split(" ").map(|l| l.parse().unwrap()).collect();

        let mut status: Status = Status::Unknown;
        let mut dir: Direction = Direction::Unknown;
        // all inputs have at least 2 levels, so we don't need a check for that
        let mut i = 0;
        let mut j = 1;
        let limit = current.len();

        loop {
            let diff = current[j] - current[i];
            let abs_diff = diff.abs();

            if abs_diff == 0 || abs_diff > 3 {
                status = Status::Unsafe;
                break;
            }

            match dir {
                Direction::Unknown => {
                    dir = if diff > 0 {
                        Direction::Increasing
                    } else {
                        Direction::Decreasing
                    }
                }
                Direction::Increasing => {
                    if diff < 0 {
                        status = Status::Unsafe;
                        break;
                    }
                }
                Direction::Decreasing => {
                    if diff > 0 {
                        status = Status::Unsafe;
                        break;
                    }
                }
            }

            i += 1;
            j += 1;

            if j >= limit {
                status = Status::Safe;
                break;
            }
        }

        statuses.push(status);
    }

    return statuses
        .iter()
        .filter(|s| matches!(s, Status::Safe))
        .count();
}

enum Status {
    Unknown,
    Safe,
    Unsafe,
}

enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn p1() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part1(contents);

        assert_eq!(result, 2);
    }
}
