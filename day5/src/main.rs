use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Reading input file failed");
    let input = input.lines();
    let input : Vec<(u32, u32)> = input.map(|line| {
        let line : String = line.chars().map(|c| match c {
            'B' => '1',
            'F' => '0',
            'L' => '0',
            'R' => '1',
            _ => c
        } as char).collect();

        let row = u32::from_str_radix(&line[0..7], 2).unwrap();
        let column = u32::from_str_radix(&line[7..], 2).unwrap();

        (row, column)
    }).collect();

    let mut seat_ids : Vec<u32> = input.iter().map(|(row, column)| row * 8 + column).collect();
    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("max is {}", max_seat_id);

    seat_ids.sort();

    let missing_seat_id = seat_ids.iter().zip(seat_ids.iter().skip(1)).find(|(&a, &b)| a + 2 == b).unwrap().0 + 1;
    println!("missing is {}", missing_seat_id);
}
