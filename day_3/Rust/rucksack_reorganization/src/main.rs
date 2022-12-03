use std::collections::HashSet;

fn part_1(input_str: &str) -> i32 {
    let rucksacks: Vec<&str> = input_str.split("\n").collect::<Vec<&str>>();

    let mut total: i32 = 0;

    for rucksack in rucksacks {
        let size: usize = rucksack.len();

        let rucksack = rucksack
            .chars()
            .collect::<Vec<char>>();
         
        let left   = rucksack[..size/2].to_vec();
        let right  = rucksack[size/2..].to_vec();

        // uhh
        let left_set: HashSet<char> = left.iter().collect::<String>().chars().collect();
        let right_set: HashSet<char> = right.iter().collect::<String>().chars().collect();

        let ch = left_set
            .iter()
            .filter(
                |c| right_set
                    .contains(c)
            )
            .collect::<String>();


        let ch: char = ch.chars().nth(0).unwrap();
        total += match true {
            a if a == ch.is_lowercase() => ch as i32 - 96,
            b if b == ch.is_uppercase() => ch as i32 - 64 + 26,
            _ => panic!("bruh"),
        };
    }
    total
}

// fn part_2(input_str: String) -> i64 {
// }


fn main() {
    println!("{}", part_1(&input_str));
    // part_2(&input_str);
}
