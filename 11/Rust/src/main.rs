use std::fs;


struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64, i64) -> i64>,
    op_num: i64,
    yes: i64,
    no: i64,
    num: i64,
    test: Box<dyn Fn(i64, i64, i64, i64) -> i64>,
    seen: i64,
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        op: impl Fn(i64, i64) -> i64 + 'static,
        op_num: i64,
        yes: i64,
        no: i64,
        num: i64,
        test: impl Fn(i64, i64, i64, i64) -> i64 + 'static,
        ) -> Self 
    {
        Self {
            items,
            operation: Box::new(op),
            op_num,
            yes,
            no,
            num,
            test: Box::new(test),
            seen: 0,
        }
    }
}


fn part_1(input_str: &str) -> i64 {

    let total = 0;

    let input_monkeys: Vec<&str> = input_str.trim().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monky in input_monkeys {
        let lines: Vec<&str> = monky.trim().split("\n").collect();

        // println!("{:?}", lines[1].trim().split(", ").collect::<Vec<&str>>());
        let items: Vec<i64> = lines[1]
            .trim()
            .split(": ")
            .last().unwrap()
            .split(", ")
            .map(
                |x| x.parse::<i64>().unwrap()
            ).collect();
        
        let op: Vec<&str> = lines[2]
            .trim()
            .split(" = ")
            .nth(1)
            .unwrap()
            .split(" ")
            .collect();

        let mut operation: fn(i64, i64) -> i64;
         
        if op[1] == "+" {
            operation = |old: i64, x: i64| old + x;
        } else {
            operation = |old: i64, x: i64| old * x;
        }
        if op[2] == "old" {
            operation = |old: i64, x: i64| old * old;
        }

        let op_num = op[2].parse::<i64>().unwrap_or(1);

        let num: i64 = lines[3].trim().split(" ").last().unwrap().parse::<i64>().unwrap();
        let tr: i64 = lines[4].trim().split(" ").last().unwrap().parse::<i64>().unwrap();
        let fl: i64 = lines[5].trim().split(" ").last().unwrap().parse::<i64>().unwrap();

        // huehuehuehuehue
        let test = |x: i64, tr: i64, fl: i64, num: i64| -> i64 {[tr, fl][((x % num) > 0) as usize]};

        let monkey = Monkey::new(items, operation, op_num, tr, fl, num, test);

        monkeys.push(monkey);
    }

    for round in 0..10_000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                // inspect 
                let mut item = (monkeys[i].operation)(monkeys[i].items[j], monkeys[i].op_num);
                monkeys[i].seen += 1;

                // sigh
                // item /= 3;

                // lcm >:(
                item %= 9699690;
                monkeys[i].items[j] = item;

                // throw
                let tr = monkeys[i].yes;
                let fl = monkeys[i].no;
                let num = monkeys[i].num;
                let monk = (monkeys[i].test)(item, tr, fl, num) as usize;
                monkeys[monk].items.push(item);
            }
            monkeys[i].items = vec![];
        }

        println!("round {}", round+1);

        for m in 0..monkeys.len() {
            println!("Monkey {m} touched stuff {} times", monkeys[m].seen);
        }
    }



    let mut activity = monkeys.iter().map(|monky| monky.seen).collect::<Vec<i64>>();
    activity.sort();
    activity.reverse();

    println!("monky business = {}", activity[0]*activity[1]);

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
