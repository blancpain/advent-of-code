use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        part_1(lines.map_while(Result::ok).collect());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1(lines: Vec<String>) -> u64 {
    let mut removed_total = 0;

    // Convert to 2D character grid once
    let mut char_grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    loop {
        let mut removed = 0;
        let copy = char_grid.clone();

        for (index, row) in char_grid.iter_mut().enumerate() {
            for (chart_index, c) in row.iter_mut().enumerate() {
                let mut count = 0;
                if *c == '@' {
                    let top = copy
                        .get(index.wrapping_sub(1))
                        .and_then(|line| line.get(chart_index).copied());
                    let bottom = copy
                        .get(index + 1)
                        .and_then(|line| line.get(chart_index).copied());
                    let left = copy
                        .get(index)
                        .and_then(|line| line.get(chart_index.wrapping_sub(1)).copied());
                    let right = copy
                        .get(index)
                        .and_then(|line| line.get(chart_index + 1).copied());
                    let top_left = copy
                        .get(index.wrapping_sub(1))
                        .and_then(|line| line.get(chart_index.wrapping_sub(1)).copied());
                    let top_right = copy
                        .get(index.wrapping_sub(1))
                        .and_then(|line| line.get(chart_index + 1).copied());
                    let bottom_left = copy
                        .get(index + 1)
                        .and_then(|line| line.get(chart_index.wrapping_sub(1)).copied());
                    let bottom_right = copy
                        .get(index + 1)
                        .and_then(|line| line.get(chart_index + 1).copied());

                    for neighbour in [
                        top,
                        bottom,
                        left,
                        right,
                        top_left,
                        top_right,
                        bottom_right,
                        bottom_left,
                    ] {
                        if neighbour.is_some_and(|x| x == '@') {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        *c = '.';
                        removed += 1;
                        removed_total += 1;
                    }
                };
            }
        }
        if removed == 0 {
            break;
        }
    }

    println!("{}", removed_total);
    removed_total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let lines = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .lines()
            .map(|x| x.to_owned())
            .collect();

        assert_eq!(part_1(lines), 43);
    }
}
