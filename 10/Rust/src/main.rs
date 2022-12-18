use std::fs;


fn part_1(input_str: &str) -> i32 {
    let mut register = 1;

    let mut current_cycle = 1;

    let lines: Vec<&str> = input_str.trim().split("\n").collect();
    let mut second_cycle = false;

    let mut i = 0;
    let mut total: i32 = 0;
    while i < lines.len() {
        let line = lines[i];
        let tokens: Vec<&str> = line.split(" ").collect();
        let instruction = tokens[0];

        match current_cycle {
            20|60|100|140|180|220 => total += register*current_cycle,
            _ => {}
        }

        if second_cycle {
            second_cycle = false;
            current_cycle += 1;
            register += tokens[1].parse::<i32>().unwrap();
            i += 1;
            continue;
        }

        if instruction == "addx" {
            second_cycle = true;
            current_cycle += 1;
        } else {
            current_cycle += 1;
            i += 1;
        }
    }
    total
}

fn part_2(input_str: &str) -> i64 {
    let mut register = 1;

    let mut current_cycle = 1;

    let lines: Vec<&str> = input_str.trim().split("\n").collect();
    let mut second_cycle = false;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let tokens: Vec<&str> = line.split(" ").collect();
        let instruction = tokens[0];

        if second_cycle {
            second_cycle = false;
            register += tokens[1].parse::<i32>().unwrap();
            i += 1;
            

            if current_cycle % 40 == register - 1
                || current_cycle % 40 == register ||
                    current_cycle % 40 == register + 1 {
                        print!("#");
                    }
            else {
                print!(".");
            }


            if current_cycle % 40 == 0 {print!("\n");}

            current_cycle += 1;

            continue;
        }

        if instruction == "addx" {
            second_cycle = true;


            if current_cycle % 40 == register - 1
                || current_cycle % 40 == register ||
                    current_cycle % 40 == register + 1 {
                        print!("#");
                    }
            else {
                print!(".");
            }
            if current_cycle % 40 == 0 {print!("\n");}


            current_cycle += 1;

        } else {
            i += 1;

            if current_cycle % 40 == register - 1
                || current_cycle  % 40 == register ||
                    current_cycle % 40 == register + 1 {
                        print!("#");
                    }
            else {
                print!(".");
            }
            if current_cycle % 40 == 0 {print!("\n");}

            current_cycle += 1;

        }
    }
    0
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    // println!("{}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
