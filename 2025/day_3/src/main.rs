use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part_1();
    let mut sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            sum += part_2(&line);
        }
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Assume you need to pick m batteries from a row of n.
// For battery k, you need to leave at least m-k+1 batteries at the end.
// So for example, if you're picking 2 batteries, like in part 1, there must be at least 1 battery between it and the end. So as long as you stop searching at n-(m-k+1), not n, you can always just look for the largest number.
fn part_2(line: &str) -> u64 {
    let mut largest_possible_number_array: Vec<u64> = Vec::with_capacity(12);
    let numbers: Vec<u64> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();

    let mut batteries_to_fill = 12;
    let numbers_len = numbers.len();

    let mut substr = &numbers[..numbers_len - batteries_to_fill + 1];
    let mut start = 0;

    while batteries_to_fill != 0 {
        // prefer below to max_by_key since the latter returns the second val if both are equal
        let max_value = *substr.iter().max().unwrap();
        let index = substr.iter().position(|&v| v == max_value).unwrap();

        largest_possible_number_array.push(max_value);
        start += index + 1;
        batteries_to_fill -= 1;
        if batteries_to_fill == 0 {
            break;
        }
        substr = &numbers[start..numbers_len - batteries_to_fill + 1];
    }

    largest_possible_number_array
        .iter()
        .fold(0, |acc, d| acc * 10 + d)
}

fn part_1() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            let numbers = line.chars().filter_map(|c| c.to_digit(10));
            let highest = numbers.clone().max().unwrap();
            let second_highest = numbers.clone().filter(|x| x != &highest).max().unwrap();

            let highest_position = numbers.clone().position(|x| x == highest).unwrap();
            let numbers_right_of_highest: Vec<u32> =
                numbers.clone().collect::<Vec<u32>>()[highest_position + 1..].to_vec();
            let line_len = line.len() - 1;

            let combined = if highest_position == line_len {
                second_highest * 10 + highest
            } else {
                match numbers_right_of_highest.is_empty() {
                    true => {
                        let second_highest = numbers_right_of_highest.iter().max().unwrap();
                        highest * 10 + second_highest
                    }
                    false => highest * 10 + second_highest,
                }
            };

            sum += combined;
        }
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_2() {
        let line1 = "987654321111111";
        let line2 = "811111111111119";
        let line3 = "234234234234278";
        let line4 = "8181 81911112111";

        assert_eq!(part_2(line1), 987654321111);
        assert_eq!(part_2(line2), 811111111119);
        assert_eq!(part_2(line3), 434234234278);
        assert_eq!(part_2(line4), 888911112111);
    }
}
