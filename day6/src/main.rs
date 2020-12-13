use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    // Split on double newline
    let input = input.split("\n\n");

    // Filter out all whitespace
    let input_sans_whitespace : Vec<String> = input.clone().map(|lines| lines.chars().filter(|c| !c.is_whitespace()).collect()).collect();

    let total_answers : usize = input_sans_whitespace.iter().map(|group_answers| {
        let mut group_answers : Vec<char> = group_answers.chars().collect();
        group_answers.sort();
        group_answers.dedup();
        group_answers.len()
    }).sum();

    println!("Total answers is {}", total_answers);

    let total_intersected_answers : usize = input.map(|group_answers| {
        let individual_answers : Vec<HashSet<char>> = group_answers.lines().map(|line| line.chars().collect()).collect();
        let iter = individual_answers.iter().skip(1);
        iter.fold(individual_answers[0].clone(), |set1, set2| set1.intersection(set2).cloned().collect()).len()
    }).sum();

    println!("Total intersected answers is {}", total_intersected_answers);
}
