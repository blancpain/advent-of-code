use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        part_1(lines.map_while(Result::ok).collect());
    }
    if let Ok(lines) = read_lines("input.txt") {
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

fn part_2(lines: Vec<String>) -> i64 {
    let mut organized_lines: Vec<Vec<String>> = Vec::new();
    let mut total_space = 0;
    let mut sum = 0;

    let op_row = &lines[lines.len() - 1];
    let col_starts: Vec<usize> = op_row
        .char_indices()
        .filter(|(_, c)| *c == '*' || *c == '+')
        .map(|(i, _)| i)
        .collect();

    let mut boundaries = col_starts.clone();
    boundaries.push(op_row.len());

    for line in lines {
        let mut temp = Vec::new();
        for window in boundaries.windows(2) {
            let slice = &line[window[0]..window[1]].trim_end();
            temp.push(slice.to_string());
        }

        for (index, item) in temp.iter().enumerate() {
            if total_space < temp.len() {
                organized_lines.resize(total_space + 1, Vec::new());
            }
            organized_lines[index].push(item.to_string());
            total_space += 1;
        }
    }

    let mut placeholder_vec = Vec::new();

    for item in organized_lines {
        let (nums, op) = item.split_at(item.len() - 1);
        let op: String = op.iter().map(|s| s.trim()).collect();
        let max_width = nums.iter().map(|n| n.trim().len()).max().unwrap();

        for (count, _) in (0..max_width).enumerate() {
            let mut temp = String::from("");
            for num in nums {
                let current_digit = num.chars().nth(count);

                if current_digit.is_some_and(|x| x.is_ascii_digit()) {
                    temp.push(current_digit.unwrap())
                }
            }
            if !temp.is_empty() {
                placeholder_vec.push(temp.parse::<i64>().unwrap());
            }
        }

        let temp_sum = match op.as_str() {
            "+" => placeholder_vec.iter().sum(),
            "*" => placeholder_vec.iter().product(),
            _ => 0,
        };

        sum += temp_sum;
        placeholder_vec.drain(..);
    }

    println!("PART 2: {}", sum);
    sum
}

fn part_1(lines: Vec<String>) -> i64 {
    let mut organized_lines: Vec<Vec<String>> = Vec::new();
    let mut total_space = 0;
    for line in lines {
        let trimmed: Vec<_> = line.split(" ").filter(|x| !x.is_empty()).collect();

        for (index, item) in trimmed.iter().enumerate() {
            if total_space < trimmed.len() {
                organized_lines.resize(total_space + 1, Vec::new());
            }
            organized_lines[index].push(item.to_string());
            total_space += 1;
        }
    }

    let sum: i64 = organized_lines
        .iter()
        .map(|x| {
            let (nums, op) = x.split_at(x.len() - 1);

            let parsed_nums: Vec<i64> =
                nums.iter().map(|num| num.parse::<i64>().unwrap()).collect();

            let op = op.join("");
            match op.as_str() {
                "+" => vec![parsed_nums.iter().sum()],
                "*" => vec![parsed_nums.iter().product()],
                _ => vec![0],
            }
        })
        .flat_map(|x| x.to_owned())
        .sum();

    println!("PART 1: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .lines()
            .map(|x| x.to_owned())
            .collect();

        assert_eq!(part_1(lines), 4277556);
    }

    #[test]
    fn test_part_2() {
        let lines = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .lines()
            .map(|x| x.to_owned())
            .collect();

        assert_eq!(part_2(lines), 3263827);
    }
}
