use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    for item in contents {
        println!("{item}");
        // do stuff
    }
}
