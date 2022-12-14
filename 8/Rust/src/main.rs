use std::fs;


fn part_1(input_str: &str) -> i32 {

    let rows = input_str
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let matrix: Vec<Vec<u32>> = rows
        .iter()
        .map(
            |row| row
            .chars()
            .map(|c| c
                 .to_digit(10)
                 .unwrap()
            )
            .collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>();

    let mut visible_trees = 0;
    let height = matrix.len();
    let width = matrix[0].len();

    for i in 0..height {
        'tree_loop :
        for j in 0..width {
            let mut l = true;
            let mut r = true;
            let mut u = true;
            let mut d = true;

            let tree = matrix[i][j];


            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                visible_trees += 1;
                continue 'tree_loop;
            }

            for k in i+1..height {
                if tree <= matrix[k][j] {
                    d = false;
                }
            }

            for k in 0..i {
                if tree <= matrix[k][j] {
                    u = false;
                }
            }

            for o in &matrix[i][j+1..width] {
                if tree <= *o {
                    r = false;
                }
            }

            for o in &matrix[i][0..j] {
                if tree <= *o {
                    l = false;
                }
            }

            if l || r || u || d  {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn part_2(input_str: &str) -> i64 {

    let rows = input_str
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let matrix: Vec<Vec<u32>> = rows
        .iter()
        .map(
            |row| row
            .chars()
            .map(|c| c
                 .to_digit(10)
                 .unwrap()
            )
            .collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>();

    let height = matrix.len();
    let width = matrix[0].len();
    let mut best = 0;

    for i in 0..height {
        for j in 0..width {
            let mut score;

            let mut l = 0;
            let mut r = 0;
            let mut u = 0;
            let mut d = 0;

            let tree = matrix[i][j];

            
            for k in i+1..height {
                d += 1;
                if tree <= matrix[k][j] {
                    break;
                }
            }

            for k in (0..i).rev() {
                u += 1;
                if tree <= matrix[k][j] {
                    break;
                }
            }
            
            for k in j+1..width {
                r += 1;
                if tree <= matrix[i][k] {
                    break;
                }
            }

            for k in (0..j).rev() {
                l += 1;
                if tree <= matrix[i][k] {
                    break;
                }
            }

            score = l * r * u * d;
            if score > best {best = score}
        }
    }
    best
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file.").trim());

    if input_str.len() == 0 {
        panic!("puzzle input string missing"); 
    }
    println!("bruh: {}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
