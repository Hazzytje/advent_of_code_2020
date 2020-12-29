use std::fs;
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::has_path_connecting;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");

    let regex = Regex::new(r"([1-9]) ([a-z]+ [a-z]+)").expect("Regex failed to compile");

    let mut graph = DiGraphMap::new();
    let mut bags = HashSet::new();

    for line in input.lines() {
        bags.insert(line.split(" bags contain").next().unwrap());
    }
    for line in input.lines() {
        let mut line_parts = line.split(" bags contain ").into_iter();
        let bag_name = line_parts.next().unwrap();
        let contained_bags = line_parts.next().unwrap();
        for captures in regex.captures_iter(contained_bags) {
            graph.add_edge(*bags.get(bag_name).unwrap(), *bags.get(&captures[2]).unwrap(), 1);
        }
    }

    let shiny_gold = bags.get("shiny gold").unwrap();
    let connected_bags = bags.iter().filter(|bag| has_path_connecting(&graph, bag, shiny_gold, None)).count();
    println!("connected bags: {}", connected_bags - 1); // - 1 because there is a path from shiny_gold to shiny_gold
}
