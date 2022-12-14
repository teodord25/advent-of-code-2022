use std::fs;


fn part_1(input_str: &str) -> i64 {
    let total = 0;



    total
}

fn part_2(input_str: &str) -> i64 {
    let total = 0;



    total
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    println!("{}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
