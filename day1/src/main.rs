use std::fs;

fn find_pair_that_sums_to(vec: &[u32], target_sum: u32) -> Option<(u32, u32)> {
    let mut index_left = 0;
    let mut index_right = vec.len() - 1;

    loop {
        if index_left >= index_right {
            return None
        }

        let left = vec[index_left];
        let right = vec[index_right];

        let sum = left + right;

        if sum == target_sum {
            return Some((left, right));
        }
        else if sum < target_sum {
            index_left += 1;
        } else { //sum > target_sum
            index_right -= 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    let input = input.lines();
    let mut input : Vec<u32> = input.map(|n| n.parse::<u32>().unwrap()).collect();
    input.sort();
    let input = input;

    match find_pair_that_sums_to(&input, 2020) {
        Some((left, right)) => println!("part 1: left: {}, right: {}, product: {}", left, right, left * right),
        None => println!("part 1: I got nothin'"),
    }

    for (index, elem) in input[0..input.len()-2].iter().enumerate() {
        match find_pair_that_sums_to(&input[index..input.len()], 2020 - elem) {
            Some((left, right)) => println!("part 2: elem: {}, left: {}, right: {}, product: {}", elem, left, right, elem * left * right),
            None => (),
        }
    }
}
