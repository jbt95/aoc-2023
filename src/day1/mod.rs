use std::{fs, path::Path};

pub fn run() {
    println!(
        "{}",
        fs::read_to_string(Path::new("./src/day1/input.txt"))
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|line| {
                line.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<Vec<char>>()
            })
            .map(|numbers| {
                vec![numbers.first().unwrap(), numbers.last().unwrap()]
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
    );
}
