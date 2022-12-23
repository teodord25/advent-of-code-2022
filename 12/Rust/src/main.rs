use std::fs;

fn print_state(map: Vec<Vec<char>>, curr: Option<[usize; 2]>) -> () {
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
    print_state(map.clone(), None);

    println!("{}", 'A' as u32);

    println!("GO\n");
    traverse(&map, start, 0);


    0
}


fn traverse(map: &Vec<Vec<char>>, current: [usize; 2], steps: usize) -> (bool, usize) {
    let x = current[0];
    let y = current[1];

    let mut steps = 0;

    if map[y][x] == 'E' {
        return (true, steps + 1)
    }

    let mut done = false;

    let mut directions: Vec<[usize;2]> = vec![];

    let mut val = map[y][x] as u32;
    if steps == 0 { val = 0; }
    println!("val {} ({val:?}) ", val as u8 as char);

    if y > 0 && x > 0 {
        if val + 1 >= map[y-1][x] as u32 { directions.push([y-1, x]); }
        if val + 1 >= map[y][x - 1] as u32 { directions.push([y, x - 1]); }
    }

    if val + 1 >= map[y+1][x] as u32 { directions.push([y+1, x]); }
    if val + 1 >= map[y][x + 1] as u32 { directions.push([y, x + 1]); }

    println!("directions {directions:?}");

    for dir in directions {
        (done, steps) = traverse(&map, dir, steps);
        if done { break; }
    }

    print_state(map.clone(), Some(current));
    println!("steps: {steps}");

    (false, steps)
}



fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    println!("{}", code(&input_str));
}
