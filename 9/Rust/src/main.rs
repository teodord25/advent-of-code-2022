use std::{fs, collections::HashSet, char::from_digit};


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

fn part_2(input_str: &str) -> i64 {
    let mut positions: HashSet<[i64;2]> = HashSet::new();

    let s = [i64::MAX/2, i64::MAX/2];
    let s = [11, 15];
    // 26 by 22

    // let mut tail = s;
    let mut head: [i64;2] = s;
    let mut knots: Vec<[i64;2]> = vec![s;9];

    // let mut dist = 0f32;
    let mut diff = [0, 0];
    let mut diffs: Vec<[i64;2]> = vec![[0;2];10];

    let moves: Vec<&str> = input_str.split("\n").collect();

    let mut map = vec![vec!['#'; 26]; 22];
    fn print_map(head: [i64;2], knots: &Vec<[i64;2]>, map: &mut Vec<Vec<char>>) {

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                map[i][j] = '.';
            }
        }

        for i in (0..knots.len()).rev() {

            let k = knots[i];
            let kx = (k[0] % 26) as usize;
            let ky = (k[1] % 22) as usize;


            map[ky][kx] = from_digit(i as u32, 10).unwrap();
        }

        map[(head[1] % 22) as usize][(head[0] % 26) as usize] = 'H';


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
    
    print_map(head, &knots, &mut map);
    for m in moves {
        let direction = m.split(" ").nth(0).unwrap();
        let number: u32 = m.split(" ").nth(1).unwrap().parse().unwrap();

        println!("== {m} ==\n");
        for _ in 0..number {
            match direction {
                "U" => {
                    // println!("{number}");
                    // println!("head {head:?}");
                    println!("diffs {:?}", diffs);
                    head[1] -= 1;
                    (diffs[0][0], diffs[0][1]) = (head[0].abs_diff(knots[0][0]) as i64, head[1].abs_diff(knots[0][1]) as i64);

                    let mut ym: i64 = 0;
                    let mut xm: i64 = 0;

                    if diffs[0][0] > 1 || diffs[0][1] > 1 {
                        if diffs[0][0] == 0 || diffs[0][1] == 0 {

                            knots[0][1] -= 1;
                            ym = -1;

                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;

                        } else {
                            xm = (head[0] - knots[0][0]) as i64;
                            knots[0][0] = head[0];
                            println!("XM {xm}");
                            println!("head {head:?}");
                            println!("knot {:?}", knots[0]);

                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;


                            knots[0][1] -= 1;
                            ym = -1;

                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;
                        }
                    }

                    for i in 1..knots.len() - 1 {
                        if diffs[i][0] > 1 || diffs[i][1] > 1 {
                            if diffs[i][0] == 0 || diffs[i][1] == 0 {
                                knots[i][1] += ym;
                                println!("diffs TRY TO [{i}][1] - 1 {:?}", diffs);
                                diffs[i][1] -= 1;
                                diffs[i+1][1] += 1;

                            } else {
                                knots[i][1] += ym;
                                knots[i][0] += xm;

                                diffs[i][1] -= 1;
                                diffs[i][0] -= 1;
                                diffs[i+1][1] += 1;
                                diffs[i+1][0] += 1;
                            }

                            println!("diffs {:?}", diffs);
                            (diffs[i+1][0], diffs[i+1][1]) = (knots[i][0].abs_diff(knots[i+1][0]) as i64, knots[i][1].abs_diff(knots[i+1][1]) as i64);
                            println!("diffs {:?}", diffs);
                            println!("knots: {knots:?}");
                            positions.insert(knots[knots.len()-1]);
                        print_map(head, &knots, &mut map);
                        }
                    }
                        print_map(head, &knots, &mut map);

                    // println!("diffs {diffs:?}");
                    //     println!("head  {head:?}");
                    //     println!("knots {knots:?}");
                }

                "D" => { 
                    head[1] += 1;
                    (diffs[0][0], diffs[0][1]) = (head[0].abs_diff(knots[0][0]) as i64, head[1].abs_diff(knots[0][1]) as i64);


                    if diffs[0][0] > 1 || diffs[0][1] > 1 {
                        if diffs[0][0] == 0 || diffs[0][1] == 0 {
                            knots[0][1] += 1;
                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;

                        } else {
                            knots[0][0] = head[0];
                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;

                            knots[0][1] += 1;
                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;
                        }
                    }

                    for i in 1..knots.len() - 1 {
                        if diffs[i][0] > 1 || diffs[i][1] > 1 {
                            if diffs[i][0] == 0 || diffs[i][1] == 0 {
                                knots[i][1] += 1;
                                diffs[i][1] -= 1;
                                diffs[i+1][1] += 1;

                            } else {
                                knots[i][0] = knots[i-1][0];
                                diffs[i][0] -= 1;
                                diffs[i+1][0] += 1;

                                knots[i][1] += 1;
                                diffs[i][1] -= 1;
                                diffs[i+1][1] += 1;
                            }

                            (diffs[i+1][0], diffs[i+1][1]) = (knots[i][0].abs_diff(knots[i+1][0]) as i64, knots[i][1].abs_diff(knots[i+1][1]) as i64);

                            positions.insert(knots[knots.len()-1]);
                        }
                    }
                        print_map(head, &knots, &mut map);
                }

                "L" => { 
                    head[0] -= 1;
                    (diffs[0][0], diffs[0][1]) = (head[0].abs_diff(knots[0][0]) as i64, head[1].abs_diff(knots[0][1]) as i64);


                    if diffs[0][0] > 1 || diffs[0][1] > 1 {
                        if diffs[0][0] == 0 || diffs[0][1] == 0 {
                            knots[0][0] -= 1;
                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;

                        } else {
                            knots[0][1] = head[0];
                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;

                            knots[0][0] -= 1;
                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;
                        }
                    }

                    for i in 1..knots.len() - 1 {
                        if diffs[i][0] > 1 || diffs[i][1] > 1 {
                            if diffs[i][0] == 0 || diffs[i][1] == 0 {
                                knots[i][0] -= 1;
                                diffs[i][0] -= 1;
                                diffs[i+1][0] += 1;

                            } else {
                                knots[i][1] = knots[i-1][0];
                                diffs[i][1] -= 1;
                                diffs[i+1][1] += 1;

                                knots[i][0] -= 1;
                                diffs[i][0] -= 1;
                                diffs[i+1][0] += 1;
                            }

                            (diffs[i+1][0], diffs[i+1][1]) = (knots[i][0].abs_diff(knots[i+1][0]) as i64, knots[i][1].abs_diff(knots[i+1][1]) as i64);

                            positions.insert(knots[knots.len()-1]);
                        }
                    }
                        print_map(head, &knots, &mut map);
                }

                "R" => { 
                    head[0] += 1;
                    (diffs[0][0], diffs[0][1]) = (head[0].abs_diff(knots[0][0]) as i64, head[1].abs_diff(knots[0][1]) as i64);

                    if diffs[0][0] > 1 || diffs[0][1] > 1 {
                        if diffs[0][0] == 0 || diffs[0][1] == 0 {
                            knots[0][0] += 1;
                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;

                        } else {
                            knots[0][1] = head[0];
                            diffs[0][1] -= 1;
                            diffs[1][1] += 1;

                            knots[0][0] += 1;
                            diffs[0][0] -= 1;
                            diffs[1][0] += 1;
                        }
                    }

                    for i in 1..knots.len() - 1 {
                        if diffs[i][0] > 1 || diffs[i][1] > 1 {
                            if diffs[i][0] == 0 || diffs[i][1] == 0 {
                                knots[i][0] += 1;
                                diffs[i][0] -= 1;
                                diffs[i+1][0] += 1;

                            } else {
                                knots[i][1] = knots[i-1][0];
                                diffs[i][1] -= 1;
                                diffs[i+1][1] += 1;

                                knots[i][0] += 1;
                                diffs[i][0] -= 1;
                                diffs[i+1][0] += 1;
                            }

                            (diffs[i+1][0], diffs[i+1][1]) = (knots[i][0].abs_diff(knots[i+1][0]) as i64, knots[i][1].abs_diff(knots[i+1][1]) as i64);

                            positions.insert(knots[knots.len()-1]);
                        }
                    }
                        print_map(head, &knots, &mut map);
                }

                _ => ()
            }
        }
        print_map(head, &knots, &mut map);
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
