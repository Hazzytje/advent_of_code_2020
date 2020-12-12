use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    let input = input.lines();

    let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").expect("Regex failed to compile");

    let input : Vec<(u32, u32, char, String)> = input.map( |line| {
        regex.captures(line).map(|captures| {
            (
                captures[1].parse::<u32>().unwrap(),
                captures[2].parse::<u32>().unwrap(),
                captures[3].chars().next().unwrap(),
                captures[4].to_string()
            )
        }).unwrap()
    }).collect();

    let num_valid_passwords = input.iter().filter(|(min, max, key, password)| {
        let key_count = password.chars().filter(|c| c == key).count();
        key_count >= (*min as usize) && key_count <= (*max as usize)
    }).count();

    println!("Part 1: Number of valid passwords: {}", num_valid_passwords);
}
