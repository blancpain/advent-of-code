use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part_1();
    part_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_2() {
    let mut sum = 0;
    let max_num_len = 12;

    // logic should be: while we still have 12 - n slots (where n is the digits we have so far) to the right for a full number we keep fetching the largest numbers

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut largest_possible_number: Vec<u32> = Vec::with_capacity(max_num_len);

            while largest_possible_number.len() < max_num_len {
                // fill up
                largest_possible_number.insert(0, 3);
            }

            // let numbers = line.chars().filter_map(|c| c.to_digit(10));
            // let highest = numbers.clone().max().unwrap();
            // let second_highest = numbers.clone().filter(|x| x != &highest).max().unwrap();
            //
            // let highest_position = numbers.clone().position(|x| x == highest).unwrap();
            // let numbers_right_of_highest: Vec<u32> =
            //     numbers.clone().collect::<Vec<u32>>()[highest_position + 1..].to_vec();
            // let line_len = line.len() - 1;
            //
            // let combined = if highest_position == line_len {
            //     second_highest * 10 + highest
            // } else {
            //     match numbers_right_of_highest.is_empty() {
            //         true => {
            //             let second_highest = numbers_right_of_highest.iter().max().unwrap();
            //             highest * 10 + second_highest
            //         }
            //         false => highest * 10 + second_highest,
            //     }
            // };

            let new_num = largest_possible_number
                .iter()
                .fold(0, |acc, d| acc * 10 + d);

            sum += new_num;
        }
    }
    println!("{}", sum);
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
