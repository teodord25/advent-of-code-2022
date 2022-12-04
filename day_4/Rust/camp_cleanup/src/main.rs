fn part_1(input_str: &str) -> i64 {
    let mut total = 0;
    let pairs = input_str.trim().split("\n");

    for pair in pairs {
        let pair = pair.trim().split(",").collect::<Vec<&str>>();

        let left = pair[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let right = pair[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if left[0] <= right[0] && left[1] >= right[1] {
            total += 1
        }

        else if right[0] <= left[0] && right[1] >= left[1] {
            total += 1
        }
    }
    total
}

fn part_2(input_str: &str) -> i64 {
    let mut total = 0;
    let pairs = input_str.trim().split("\n");

    for pair in pairs {
        let pair = pair.trim().split(",").collect::<Vec<&str>>();

        let left = pair[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let right = pair[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if !(left[1] < right[0] || left[0] > right[1]) {
            total += 1
        }
    }
    total
}

fn main() {
    let input_str = String::from("");

    println!("{}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
