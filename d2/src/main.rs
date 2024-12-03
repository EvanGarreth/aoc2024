use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    println!("Day 02");

    let p1_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p1_result = part1(p1_contents);

    println!("Part 1 result: {p1_result} safe reports");

    let p2_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p2_result = part2(p2_contents);

    println!("Part 2 result: {p2_result} safe reports");

    return Ok(());
}

fn part1(reader: BufReader<File>) -> usize {
    let mut statuses: Vec<Status> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let current: Vec<i32> = line.split(" ").map(|l| l.parse().unwrap()).collect();

        statuses.push(get_status(&current));
    }

    return statuses
        .iter()
        .filter(|s| matches!(s, Status::Safe))
        .count();
}

fn part2(reader: BufReader<File>) -> usize {
    let mut reports: Vec<Vec<i32>> = vec![];
    let mut statuses: Vec<Status> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let current: Vec<i32> = line.split(" ").map(|l| l.parse().unwrap()).collect();
        reports.push(current);
    }

    // bruteforce the heck out of it
    for report in reports {
        if matches![get_status(&report), Status::Safe] {
            statuses.push(Status::Safe);
            continue;
        }

        let mut queue: VecDeque<Vec<i32>> = VecDeque::new();

        for i in 0..report.len() {
            let mut once_removed = report.clone();
            once_removed.remove(i);
            queue.push_back(once_removed);
        }

        loop {
            let current = queue.pop_front().unwrap();

            if matches![get_status(&current), Status::Safe] {
                statuses.push(Status::Safe);
                break;
            }

            if queue.len() == 0 {
                statuses.push(Status::Unsafe);
                break;
            }
        }
    }

    return statuses
        .iter()
        .filter(|s| matches!(s, Status::Safe))
        .count();
}

fn get_status(report: &Vec<i32>) -> Status {
    let mut status: Status = Status::Unknown;
    let mut dir: Direction = Direction::Unknown;
    // all inputs have at least 2 levels, so we don't need a check for that
    let mut i = 0;
    let mut j = 1;
    let limit = report.len();

    loop {
        let diff = report[j] - report[i];
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

    return status;
}

enum Status {
    Unknown,
    Safe,
    Unsafe,
}

#[derive(Clone, Copy)]
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

    #[test]
    fn p2() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part2(contents);

        assert_eq!(result, 4);
    }
}
