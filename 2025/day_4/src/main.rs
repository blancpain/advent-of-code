use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            part_1(&line);
        }
    }
}

// for each paper roll check positions [current_row] + 1 and [current_row] -1 (minding start and end - 0 and .len())
// AND [row_above] + 1, [row_above] - 1 (==== / ====)
// AND [row_below] + 1, [row_below] -1 (==== / ====)

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1(line: &str) -> u64 {
    println!("{}", line);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let lines = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_1(lines), 13);
    }
}
