use std::fs;
use std::collections::HashMap;
use nalgebra::base::Matrix;
use nalgebra::base::Dynamic;
use nalgebra::base::VecStorage;
use nalgebra::Complex;

fn part1(input : &Vec<u32>) {
    let mut diff_count : HashMap<u32, u32> = HashMap::new();
    for diff in input.iter().zip(input.iter().skip(1)).map(|(a, b)| b - a) {
        *diff_count.entry(diff).or_insert(0) += 1;
    }

    for (diff, count) in &diff_count {
        println!("diff: {}, count: {}", diff, count);
    }
    println!("sum: {}", diff_count.get(&1).unwrap() * (diff_count.get(&3).unwrap() + 1));
}

fn part2(input: &Vec<u32>) {
    let mut mat = Matrix::<Complex<f64>, Dynamic, Dynamic, VecStorage<Complex<f64>, Dynamic, Dynamic>>::zeros(input.len(), input.len());

    for (i, ioltage) in input.iter().enumerate() {
        for (j, joltage) in input.iter().skip(i + 1).enumerate() {
            // mat[(i, i + 1 + j)] = vec!(1, 2, 3).contains(&(joltage - ioltage)) ? 1f64 : 0f64;
            if vec!(1, 2, 3).contains(&(joltage - ioltage)) {
                mat[(i, i + 1 + j)] = Complex{re: 1f64, im: 0f64};
            }
        }
    }

    let orig_mat = mat.clone();

    // let mut paths = 0u64;
    // while mat.sum() != 0 {
    //     mat *= &orig_mat;
    //     paths += mat[(0, input.len() - 1)];
    //     println!("adding paths: {}", mat[(0, input.len() - 1)]);
    // }

    let paths = (Matrix::<Complex<f64>, Dynamic, Dynamic, VecStorage<Complex<f64>, Dynamic, Dynamic>>::identity(input.len(), input.len()) - mat).try_inverse().unwrap()[(0, input.len() - 1)];

    println!("total paths: {}", paths);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    let input = input.lines();
    let mut input : Vec<u32> = input.map(|n| n.parse::<u32>().unwrap()).collect();
    input.push(0);
    input.sort();
    let input = input;

    part1(&input);
    part2(&input);
}
