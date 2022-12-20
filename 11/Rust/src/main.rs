use std::fs;


struct Monkey {
    items: Vec<u128>,
    operation: Box<dyn Fn(u128, u128) -> u128>,
    op_num: u128,
    yes: u128,
    no: u128,
    num: u128,
    test: Box<dyn Fn(u128, u128, u128, u128) -> u128>,
    seen: u128,
}

impl Monkey {
    fn new(
        items: Vec<u128>,
        op: impl Fn(u128, u128) -> u128 + 'static,
        op_num: u128,
        yes: u128,
        no: u128,
        num: u128,
        test: impl Fn(u128, u128, u128, u128) -> u128 + 'static,
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


fn part_2(input_str: &str) -> u128 {

    let total = 0;

    let input_monkeys: Vec<&str> = input_str.trim().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monky in input_monkeys {
        let lines: Vec<&str> = monky.trim().split("\n").collect();

        // println!("{:?}", lines[1].trim().split(", ").collect::<Vec<&str>>());
        let items: Vec<u128> = lines[1]
            .trim()
            .split(": ")
            .last().unwrap()
            .split(", ")
            .map(
                |x| x.parse::<u128>().unwrap()
            ).collect();
        
        let op: Vec<&str> = lines[2]
            .trim()
            .split(" = ")
            .nth(1)
            .unwrap()
            .split(" ")
            .collect();

        let mut operation: fn(u128, u128) -> u128;
         
        if op[1] == "+" {
            operation = |old: u128, x: u128| old + x;
        } else {
            operation = |old: u128, x: u128| old * x;
        }
        if op[2] == "old" {
            operation = |old: u128, x: u128| old * old;
        }

        let op_num = op[2].parse::<u128>().unwrap_or(1);

        let num: u128 = lines[3].trim().split(" ").last().unwrap().parse::<u128>().unwrap();
        let tr: u128 = lines[4].trim().split(" ").last().unwrap().parse::<u128>().unwrap();
        let fl: u128 = lines[5].trim().split(" ").last().unwrap().parse::<u128>().unwrap();

        // huehuehuehuehue
        let test = |x: u128, tr: u128, fl: u128, num: u128| -> u128 {[tr, fl][((x % num) > 0) as usize]};

        let monkey = Monkey::new(items, operation, op_num, tr, fl, num, test);

        monkeys.push(monkey);
    }

    for round in 0..20 {
        for i in 0..monkeys.len() {
            // println!("\nmonky {i} {:?}", monkeys[i].items);
            for j in 0..monkeys[i].items.len() {
                // inspect 
                // print!("  oh no scary {}", monkeys[i].items[j]);
                let mut item = monkeys[i].items[j];
                let num = monkeys[i].num;


                // item %= num;
                item = (monkeys[i].operation)(item, num);
                monkeys[i].seen += 1;
                // item %= num;
                item /= num;
                
                // println!("-> {item}");


                // let bruh = item;
                // item -= item - (item % num);
                // println!("{bruh} (rem: {}) -{num}> {item} (rem: {})", bruh % num, item % num);
                monkeys[i].items[j] = item;

                // throw
                let tr = monkeys[i].yes;
                let fl = monkeys[i].no;
                // println!("div? {num} yes? {tr} no {fl}");


                let monk = (monkeys[i].test)(item, tr, fl, num) as usize;
                // println!("OK {item} go to monky {monk}");
                monkeys[monk].items.push(item);
            }
            monkeys[i].items = vec![];
        }

        match round {
            0|1|2|19 => {
                println!("round {}", round+1);

                for m in 0..monkeys.len() {
                    println!("Monkey {m} touched stuff {} times", monkeys[m].seen);
                    // println!("Monkey {m} touched stuff {} times {:?}", monkeys[m].seen, monkeys[m].items);
                }
            },
            _ => {}
        }
    }



    let mut activity = monkeys.iter().map(|monky| monky.seen).collect::<Vec<u128>>();
    activity.sort();
    activity.reverse();

    println!("monky business = {}", activity[0]*activity[1]);

    total
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    println!("{}", part_2(&input_str));
}
