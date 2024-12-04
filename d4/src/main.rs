use std::{
    char,
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    println!("Day 03");

    let p1_contents = BufReader::new(File::open("src/input/data.txt")?);

    let p1_result = part1(p1_contents);

    println!("Part 1 result: {p1_result}");

    return Ok(());
}

fn part1(reader: BufReader<File>) -> i32 {
    let graph: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let rows = graph.len();
    let cols = graph[0].len();

    let mut matches = 0;

    for (i, row) in graph.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != 'X' {
                continue;
            }

            // can check up
            if i >= 3 {
                // vertical
                if graph[i - 1][j] == 'M' && graph[i - 2][j] == 'A' && graph[i - 3][j] == 'S' {
                    matches = matches + 1;
                }

                // left upper diagonal
                if j >= 3
                    && graph[i - 1][j - 1] == 'M'
                    && graph[i - 2][j - 2] == 'A'
                    && graph[i - 3][j - 3] == 'S'
                {
                    matches = matches + 1;
                }

                // right upper diagonal
                if j + 3 < cols
                    && graph[i - 1][j + 1] == 'M'
                    && graph[i - 2][j + 2] == 'A'
                    && graph[i - 3][j + 3] == 'S'
                {
                    matches = matches + 1;
                }
            }

            // can check down
            if i + 3 < rows {
                // vertical
                if graph[i + 1][j] == 'M' && graph[i + 2][j] == 'A' && graph[i + 3][j] == 'S' {
                    matches = matches + 1;
                }

                // left lower diagonal
                if j >= 3
                    && graph[i + 1][j - 1] == 'M'
                    && graph[i + 2][j - 2] == 'A'
                    && graph[i + 3][j - 3] == 'S'
                {
                    matches = matches + 1;
                }

                // right lower diagonal
                if j + 3 < cols
                    && graph[i + 1][j + 1] == 'M'
                    && graph[i + 2][j + 2] == 'A'
                    && graph[i + 3][j + 3] == 'S'
                {
                    matches = matches + 1;
                }
            }

            // can check left (backwards)
            if j >= 3 && graph[i][j - 1] == 'M' && graph[i][j - 2] == 'A' && graph[i][j - 3] == 'S'
            {
                matches = matches + 1;
            }

            // can check right
            if j + 3 < cols
                && graph[i][j + 1] == 'M'
                && graph[i][j + 2] == 'A'
                && graph[i][j + 3] == 'S'
            {
                matches = matches + 1;
            }
        }
    }

    return matches;
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn p1() {
        let contents = BufReader::new(File::open("src/input/test.txt").unwrap());

        let result = part1(contents);

        assert_eq!(result, 18);
    }

}
