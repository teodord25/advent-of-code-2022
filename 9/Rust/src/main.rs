use std::{fs, collections::HashSet, char::from_digit, borrow::{Borrow, BorrowMut}};


fn part_1(input_str: &str) -> i64 {
    let mut positions: HashSet<[u32;2]> = HashSet::new();

    let s = [u32::MAX/2, u32::MAX/2];
    // let s = [0, 4];
    let mut head = s;
    let mut tail = s;
    // let mut dist = 0f32;
    let mut diff = [0, 0];


    let moves: Vec<&str> = input_str.split("\n").collect();

    let mut map = vec![vec!['#'; 6]; 5];
    fn print_map(h: [u32;2], t: [u32;2], map: &mut Vec<Vec<char>>) {


        let hx = (h[0] % 6) as usize;
        let hy = (h[1] % 5) as usize;
        let tx = (t[0] % 6) as usize;
        let ty = (t[1] % 5) as usize;

        // println!("Head {:?} Tail {:?}", (hx, hy), (tx, ty));

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                map[i][j] = '.';
            }
        }

        map[ty][tx] = 'T';
        map[hy][hx] = 'H';

        println!("{}\n",
            map
            .iter()
            .map(
                |row| row
                .iter()
                .map(
                    |c| c
                    .to_string()
                )
                .collect::<Vec<String>>()
                .join("")
            )
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
    

    print_map(head, tail, &mut map);
    for m in moves {
        let direction = m.split(" ").nth(0).unwrap();
        let number: u32 = m.split(" ").nth(1).unwrap().parse().unwrap();

        println!("== {m} ==\n");
        for _ in 0..number {

            // println!("diff {diff:?}");
            match direction {
                "U" => {
                    head[1] -= 1;
                    (diff[0], diff[1]) = (head[0].abs_diff(tail[0]), head[1].abs_diff(tail[1]));

                    if diff[0] > 1 || diff[1] > 1 {
                        if diff[0] == 0 || diff[1] == 0 {
                            tail[1] -= 1;
                            diff[1] -= 1;

                        } else {
                            tail[0] = head[0];
                            diff[0] -= 1;

                            tail[1] -= 1;
                            diff[1] -= 1;
                        }

                        positions.insert(tail);
                    }
                }

                "D" => { 
                    head[1] += 1;
                    (diff[0], diff[1]) = (head[0].abs_diff(tail[0]), head[1].abs_diff(tail[1]));

                    if diff[0] > 1 || diff[1] > 1 {
                        if diff[0] == 0 || diff[1] == 0 {
                            tail[1] += 1;
                            diff[1] -= 1;

                        } else {
                            tail[0] = head[0];
                            diff[0] -= 1;

                            tail[1] += 1;
                            diff[1] -= 1;
                        }

                        positions.insert(tail);
                    }
                }

                "L" => { 
                    head[0] -= 1;
                    (diff[0], diff[1]) = (head[0].abs_diff(tail[0]), head[1].abs_diff(tail[1]));

                    if diff[0] > 1 || diff[1] > 1 {
                        if diff[0] == 0 || diff[1] == 0 {
                            tail[0] -= 1;
                            diff[0] -= 1;

                        } else {
                            tail[1] = head[1];
                            diff[1] -= 1;

                            tail[0] -= 1;
                            diff[0] -= 1;
                        }

                        positions.insert(tail);
                    }
                }

                "R" => { 
                    head[0] += 1;
                    (diff[0], diff[1]) = (head[0].abs_diff(tail[0]), head[1].abs_diff(tail[1]));

                    if diff[0] > 1 || diff[1] > 1 {
                        if diff[0] == 0 || diff[1] == 0 {
                            tail[0] += 1;
                            diff[0] += 1;

                        } else {
                            tail[1] = head[1];
                            diff[1] -= 1;

                            tail[0] += 1;
                            diff[0] -= 1;
                        }

                        positions.insert(tail);
                    }
                }

                _ => ()
            }
            print_map(head, tail, &mut map);
        }
    }

    positions.len() as i64
}


struct Knot {
    x: i64,
    y: i64,
}

impl Knot {
    fn new() -> Knot {
        Knot {x: 0, y: 0}
    }

    fn up(&mut self) { self.y -= 1; }

    fn down(&mut self) { self.y += 1; }

    fn left(&mut self) { self.x -= 1; }

    fn right(&mut self) { self.x += 1; }

    fn move_to(&mut self, x: i64, y: i64) { self.x = x; self.y = y; }

    fn follow(&mut self, knot: Knot) -> () {
        let delta_x = knot.x - self.x;
        let delta_y = knot.y - self.y;

        if delta_y.abs() > 1 && delta_x.abs() > 1 {
            self.x += delta_x - delta_x.signum();
            self.y += delta_y - delta_y.signum();
        }
           
        else if delta_x.abs() > 1 {
            self.x += delta_x - delta_x.signum();

            if delta_y.abs() > 0 {
                self.y += delta_y;
            }
        }

        else if delta_y.abs() > 1 {
            self.y += delta_y - delta_y.signum();

            if delta_x.abs() > 0 {
                self.x += delta_x;
            }
        }
    }
}


fn print_map(knots: &Vec<Knot>) {
    let mut map = vec![vec!['#'; 26]; 22];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            map[i][j] = '.';
        }
    }

    fn modulo(a: i64, b: i64) -> i64 {
        ((a % b) + b) % b
    }

    for (i, knot) in knots.iter().enumerate().rev() {
        let kx = modulo(knot.x + 11, 26) as usize;
        let ky = modulo(knot.y + 15, 22) as usize;

        map[ky][kx] = from_digit(i as u32, 10).unwrap();
    }

    let head = &knots[0];
    let hx = modulo(head.x + 11, 26) as usize;
    let hy = modulo(head.y + 15, 22) as usize;
    map[hy][hx] = 'H';

    println!("{}\n",
        map.iter().map(
            |row| row.iter().map(
                |c| c .to_string()).collect::<Vec<String>>().join("")
        ).collect::<Vec<String>>().join("\n"));
}
    


fn part_2(input_str: &str) -> i64 {
    let mut knots: Vec<Knot> = (0..10).map(|_| Knot::new()).collect();
    let mut positions: HashSet<[i64; 2]> = HashSet::new();

    let moves: Vec<&str> = input_str.split("\n").collect();

    print_map(&knots);

    for m in moves {
        let direction = m.split(" ").nth(0).unwrap();
        let steps: i64 = m.split(" ").nth(1).unwrap().parse().unwrap();

        println!("== {m} ==\n");
        for _ in 0..steps {
            match direction {
                "U" => {
                    knots[0].up();
                }

                "D" => { 
                    knots[0].down();
                }

                "L" => { 
                    knots[0].left();
                }

                "R" => { 
                    knots[0].right();
                }

                _ => ()
            }

                let l = knots.len();

                for i in 1..l {
                    let x = knots[i-1].x;
                    let y = knots[i-1].y;
                    let mut prev = Knot::new();
                    prev.move_to(x, y);

                    let curr = &mut knots[i];
                    curr.follow(prev);
                }

            let i = knots.len() - 1;
            let k: &Vec<Knot> = knots.borrow();
            let x = k[i].x;
            let y = k[i].y;

            positions.insert([x, y]);

        print_map(&knots);
        }
    }

    positions.len() as i64
}


fn main() {
    let input_str: String = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));
    let input_str = input_str.trim();

    if input_str.trim().len() == 0 {
        panic!("puzzle input string missing"); 
    }
    // println!("{}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
