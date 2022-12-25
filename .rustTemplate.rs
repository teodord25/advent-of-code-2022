use std::fs;


fn part_1(input_str: &str) {
}

fn part_2(input_str: &str) {
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file.")).trim();

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    part_1(&input_str);
    part_2(&input_str);
}
