use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        // part_1(lines.map_while(Result::ok).collect());
        part_2(lines.map_while(Result::ok).collect());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: below is not feasible for large ranges, we need to merge the ranges first and then basically get the len of the merged final range.
fn part_2(lines: Vec<String>) -> u128 {
    let mut count: u128 = 0;

    let break_position = lines.iter().position(|x| x.is_empty()).unwrap();
    let (ranges, _ingredients) = lines.split_at(break_position);

    let mut ranges = ranges.to_vec();

    ranges.sort_by_key(|x| {
        let mut it = x.split('-');
        let (Some(left), _) = (it.next(), it.next()) else {
            panic!("Invalid range");
        };
        left.parse::<u64>().unwrap()
    });
    ranges.dedup_by_key(|x| x.clone());

    let mut max = 0;
    for range in ranges {
        let mut it = range.split('-');
        let (Some(left), Some(right)) = (it.next(), it.next()) else {
            panic!("Invalid range");
        };

        let mut start: u128 = left.parse::<u128>().unwrap();
        let end: u128 = right.parse::<u128>().unwrap() + 1;

        if max > start {
            start = max
        }

        if max >= end {
            continue;
        }

        count += end - start;
        max = end;
    }

    println!("{}", count);
    count
}
fn part_1(lines: Vec<String>) -> usize {
    let mut fresh_ingredients = HashSet::new();

    let break_position = lines.iter().position(|x| x.is_empty()).unwrap();
    let (ranges, ingredients) = lines.split_at(break_position);
    let ingredients = &ingredients[1..];

    for ingredient in ingredients {
        for range in ranges {
            let mut it = range.split('-');
            let (Some(left), Some(right)) = (it.next(), it.next()) else {
                panic!("Invalid range");
            };

            let start = left.parse::<u64>().unwrap();
            let end = right.parse::<u64>().unwrap();

            if (start..=end).contains(&ingredient.parse::<u64>().unwrap()) {
                fresh_ingredients.insert(ingredient);
            }
        }
    }

    println!("{}", fresh_ingredients.len());
    fresh_ingredients.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let lines = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .lines()
        .map(|x| x.to_owned())
        .collect();

        assert_eq!(part_1(lines), 3);
    }

    #[test]
    fn test_part_2() {
        let lines = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .lines()
        .map(|x| x.to_owned())
        .collect();

        assert_eq!(part_2(lines), 14);
    }
}
