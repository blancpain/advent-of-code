use std::{collections::HashSet, fs};

fn main() {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    // part_1(contents);
    part_2(contents);
}

fn part_2(contents: Vec<String>) {
    let mut nums: HashSet<i64> = HashSet::new();

    for item in contents {
        let mut it = item.split('-');
        let (Some(left), Some(right)) = (it.next(), it.next()) else {
            panic!("Invalid range");
        };

        let start = left.parse::<i64>().unwrap();
        let end = right.parse::<i64>().unwrap();

        for n in start..=end {
            let current_num_str = n.to_string();
            let current_num_len = current_num_str.len();

            for m in 0..current_num_len {
                let current_pattern = &current_num_str[0..m];
                let rest_of_str = &current_num_str[m..];
                let matches: Vec<&str> = rest_of_str.matches(current_pattern).collect();

                let matches_str = matches.join("");
                let full_str = format!("{}{}", current_pattern, matches_str);

                if full_str == current_num_str {
                    println!(
                        "PATTERN: {}, FULL STR: {}; CURRENT STR:{};",
                        current_pattern, full_str, current_num_str,
                    );
                    nums.insert(n);
                }
            }
        }
    }

    let total: i64 = nums.iter().sum();
    println!("{}", total);
}

fn part_1(contents: Vec<String>) {
    let mut sum = 0;

    for item in contents {
        // do stuff
        let mut it = item.split('-');
        let (Some(left), Some(right)) = (it.next(), it.next()) else {
            panic!("Invalid range");
        };

        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();

        for n in left..=right {
            let num_length = n.to_string().len();
            if num_length % 2 != 0 {
                continue;
            };

            let str_n = n.clone().to_string();
            let (left_side, right_side) = str_n.split_at(num_length / 2);

            if left_side.parse::<i64>().unwrap() == right_side.parse::<i64>().unwrap() {
                sum += n;
            }
        }
    }
    println!("{}", sum);
}
