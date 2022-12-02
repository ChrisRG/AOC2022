use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut calorie_list = parse_file("input.txt");
    calorie_list.sort();
    println!("Part 1: {}", calorie_list[calorie_list.len() - 1]);

    let top_three: u32 = calorie_list[calorie_list.len() - 3..].iter().sum();
    println!("Part 2: {}", top_three);
}

fn parse_file(file_name: &str) -> Vec<u32> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut calorie_list: Vec<u32> = Vec::new();
    let mut single_calories: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            calorie_list.push(single_calories);
            single_calories = 0;
        } else {
            single_calories += line.parse::<u32>().unwrap();
        }
    }
    calorie_list
}
