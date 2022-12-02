fn part_1(input_str: &str) -> i64 {
    let mut sums =  input_str
        .split("\n\n")
        .map(
            |elf| elf
            .split("\n")
            .map(|snack| snack
                 .parse::<i64>()
                 .unwrap())
            .sum()
        ).collect::<Vec<i64>>();

    sums.sort();
    *sums.last().unwrap()
}

fn part_2(input_str: &str) -> i64 {
    let mut sums =  input_str
        .split("\n\n")
        .map(
            |elf| elf
            .split("\n")
            .map(|snack| snack
                 .parse::<i64>()
                 .unwrap())
            .sum()
        ).collect::<Vec<i64>>();

    sums.sort();
    sums[sums.len() - 3 ..].iter().sum()
}


fn main() {
    // println!("{}", part_1(&input_str));
    // println!("{}", part_2(&input_str));
}

