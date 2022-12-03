use std::collections::HashMap;

fn part_1(input_str: &str) -> i64 {
    /*
     * A beats Z
     * B beats X
     * C beats Y
     */

    let mut total = 0;
    let pairs = input_str.split("\n");
    
    let mut index_map = HashMap::new();
    index_map.insert("A", 0);
    index_map.insert("B", 1);
    index_map.insert("C", 2);

    for pair in pairs {
        let opponent_shape = pair.split_whitespace().nth(0).unwrap();
        let my_shape = pair.split_whitespace().nth(1).unwrap();

        let shape_value;
        let my_index;
        match my_shape {
            "X" => {shape_value = 1; my_index = 0},
            "Y" => {shape_value = 2; my_index = 1},
            "Z" => {shape_value = 3; my_index = 2},
            _ => panic!("bruh"),
        };

        let opponent_index = *index_map.get(opponent_shape).unwrap();

        let game_result = match opponent_index {
            a if a ==  my_index          => 3,
            b if b == (my_index + 1 + 3) % 3 => 0,
            c if c == (my_index - 1 + 3) % 3 => 6,
            _ => 0,
        };

        total += shape_value + game_result;
    }
    total
}

fn part_2(input_str: &str) -> i64 {

    /*
     * X -> pick losing shape
     * Y -> pick draw
     * Z -> pick win
     */

    let mut total = 0;
    let pairs = input_str.split("\n");
    for pair in pairs {
        let opponent_shape = pair.split_whitespace().collect::<Vec<&str>>()[0];
        let my_result = pair.split_whitespace().collect::<Vec<&str>>()[1];

        let mut my_shape = "";
        let mut game_result = 0;
        
        if opponent_shape == "A" {
            if my_result == "X" {
                game_result = 0;
                my_shape = "S";
            }

            if my_result == "Y" {
                game_result = 3;
                my_shape = "R";
            }

            if my_result == "Z" {
                game_result = 6;
                my_shape = "P";
            }

        }

        if opponent_shape == "B" {
            if my_result == "X" {
                game_result = 0;
                my_shape = "R";
            }

            if my_result == "Y" {
                game_result = 3;
                my_shape = "P";
            }

            if my_result == "Z" {
                game_result = 6;
                my_shape = "S";
            }

        }

        if opponent_shape == "C" {
            if my_result == "X" {
                game_result = 0;
                my_shape = "P";
            }

            if my_result == "Y" {
                game_result = 3;
                my_shape = "S";
            }

            if my_result == "Z" {
                game_result = 6;
                my_shape = "R";
            }

        }

        let shape_value = match my_shape {
            "R" => 1,
            "P" => 2,
            "S" => 3,
            _ => 0,
        };


        total += shape_value;
        total += game_result;
    }
    total
}

fn main() {
// "
    println!("{}", part_1(&input_str));
    // println!("{}", part_2(&input_str));
}
