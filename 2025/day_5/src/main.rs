use std::collections::HashSet;
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

        assert_eq!(part_1(lines), 3);
    }
}
