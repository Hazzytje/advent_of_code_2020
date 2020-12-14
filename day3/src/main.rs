use std::fs;

fn exist_tree_at(grid: &Vec<Vec<char>>, x: u32, y: u32) -> bool {
    grid[y as usize][(x as usize) % grid[0].len()] == '#'
}

fn trees_in_the_way(grid: &Vec<Vec<char>>, slope_x: u32, slope_y: u32) -> u32 {
    let mut x = 0u32;
    let mut y = 0u32;

    let mut trees = 0u32;
    while (y as usize) < grid.len() {
        if exist_tree_at(&grid, x, y) {
            trees = trees + 1;
        }

        x = x + slope_x;
        y = y + slope_y;
    }

    trees
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    let input = input.lines();
    let input : Vec<Vec<char>> = input.map(|line| {
        line.chars().collect()
    }).collect();

    let trees = trees_in_the_way(&input, 3, 1);

    println!("number of trees in the way: {}", trees);

    let trees_mut : u32 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter().map(|(slope_x, slope_y)| {
        trees_in_the_way(&input, slope_x, slope_y)
    }).product();

    println!("number of trees mut in the way: {}", trees_mut);
}
