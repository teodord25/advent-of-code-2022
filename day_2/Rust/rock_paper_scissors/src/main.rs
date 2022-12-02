fn part_1(input_str: &str) -> i64 {
    /*
     * A beats Z
     * B beats X
     * C beats Y
     */

    let mut total = 0;
    let pairs = input_str.split("\n");
    for pair in pairs {
        let opponent_shape = pair.split_whitespace().collect::<Vec<&str>>()[0];
        let my_shape = pair.split_whitespace().collect::<Vec<&str>>()[1];

        let shape_value = match my_shape {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        let mut game_result = 0;
        if opponent_shape == "A" {
            if my_shape == "X" {game_result = 3}
            if my_shape == "Y" {game_result = 6}
            if my_shape == "Z" {game_result = 0}
        }

        if opponent_shape == "B" {
            if my_shape == "X" {game_result = 0}
            if my_shape == "Y" {game_result = 3}
            if my_shape == "Z" {game_result = 6}
        }

        if opponent_shape == "C" {
            if my_shape == "X" {game_result = 6}
            if my_shape == "Y" {game_result = 0}
            if my_shape == "Z" {game_result = 3}
        }
        total += shape_value;
        total += game_result;
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
    println!("{}", part_1(&input_str));
    println!("{}", part_2(&input_str));
}
