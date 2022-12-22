use std::fs;

fn print_state(map: Vec<Vec<char>>, curr: Option<[u32; 2]>) -> () {
    let mut map = map.clone();
    if let Some(coords) = curr {
        map[coords[1] as usize][coords[0] as usize] = 'X';
    }

        let strmap = map
            .iter()
            .map(
                |chars| chars
                .iter()
                .collect::<String>() + "\n"
            ).collect::<String>();

        println!("{strmap}");
}


fn code(input_str: &str) -> i64 {
    let input_str = input_str.trim();

    let map: Vec<Vec<char>> = input_str
        .trim()
        .split("\n")
        .map(
            |line| line
            .chars()
            .collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>();

    let mut start = [0, 0];
    let mut end = [0, 0];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                'S' => { start = [j, i] }
                'E' => { end = [j, i] }
                _ => {}
            }
        }
    }

    println!("{start:?} {end:?}");
    print_state(map, None);

    0
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    println!("{}", code(&input_str));
}
