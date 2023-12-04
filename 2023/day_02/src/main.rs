
fn sum_id_of_possible_games(input: &str, max_reds: i32, max_greens: i32, max_blues: i32) -> (i32, i32) {
    // for each game (line), get the max of each color
    // store the maxes as elements of the game id 
    //      NOTE the shift between game number (1-based) and index (0-based)

    let mut game_max_reds: Vec<i32> = Vec::new();
    let mut game_max_greens: Vec<i32> = Vec::new();
    let mut game_max_blues: Vec<i32> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let mut parts = line.split(':');
        let game_part = parts.next().unwrap();

        if game_part.starts_with("Game") {
            let game_number = game_part[5..].trim().parse::<i32>().unwrap() - 1;
            game_max_reds.push(0);
            game_max_greens.push(0);
            game_max_blues.push(0);

            let rounds = parts.next().unwrap().split(';');
            for round in rounds {
                let round = round.trim();
                let colors = round.split(',');
                for color_info in colors {
                    let color_info = color_info.trim();
                    let mut color_info = color_info.split(' ');

                    let count = color_info.next().unwrap().parse::<i32>().unwrap();
                    let color: &str = color_info.next().unwrap();
                    match color {
                        "red" => {
                            if count > game_max_reds[game_number as usize] {
                                game_max_reds[game_number as usize] = count;
                            }
                        },
                        "green" => {
                            if count > game_max_greens[game_number as usize] {
                                game_max_greens[game_number as usize] = count;
                            }
                        },
                        "blue" => {
                            if count > game_max_blues[game_number as usize] {
                                game_max_blues[game_number as usize] = count;
                            }
                        },
                        _ => panic!("Invalid color"),
                    }
                }
            }
        }      
    }

    // for each game, if the maxes are less than the maxes allowed, add the game id to the sum
    let mut sum_of_game_ids: i32 = 0;
    for i in 0..game_max_reds.len() {
        if game_max_reds[i] <= max_reds && game_max_greens[i] <= max_greens && game_max_blues[i] <= max_blues {
            sum_of_game_ids += i as i32 + 1;
        }
    }

    // compute and sum the 'powers' of the games
    let mut sum_of_game_powers: i32 = 0;
    for i in 0..game_max_reds.len() {
        let power = game_max_reds[i]*game_max_greens[i]*game_max_blues[i];
        sum_of_game_powers += power;
    }

    return (sum_of_game_ids, sum_of_game_powers);
}

fn main() {
    // read input file
    let input = include_str!("../input.txt");
    let (sum_possible, sum_power) = sum_id_of_possible_games(input, 12, 13, 14);
    print!("{} - {}\n", sum_possible, sum_power);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_id_of_possible_games() {
        assert_eq!(sum_id_of_possible_games(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            12, 13, 14)
        , 8);
    }

}
