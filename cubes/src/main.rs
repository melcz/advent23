use std::fs;
use core::str::Lines;
use std::cmp::max;

fn main() {
    let input_path = "./input.txt";

    println!("In file {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();

    // calculate_sum_games(lines);
    calculate_power_cubes(lines);
}

fn calculate_power_cubes(mut lines : Lines) {
    let _config = parse_game(lines.next().unwrap());

    let mut min_cubes_per_game = Vec::new();

    for line in lines {
        let (_, game_draws) = line.split_once(":").unwrap();

        let mut min_game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for draw in game_draws.split(";") {
            let game_draw = parse_game(draw);
            min_game.red = max(min_game.red, game_draw.red);
            min_game.green = max(min_game.green, game_draw.green);
            min_game.blue = max(min_game.blue, game_draw.blue);
        }
        // print(&min_game);
        let game_power = power(&min_game);
        println!("{}", game_power);
        min_cubes_per_game.push(game_power);

    }

    let sum = min_cubes_per_game.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
}

fn calculate_sum_games(mut lines : Lines) {
    let config = parse_game(lines.next().unwrap());
    print(&config);

    let mut valid_games = Vec::new();

    for line in lines {
        let (game_header, game_draws) = line.split_once(":").unwrap();
        let game_id = parse_game_id(game_header);

        let mut is_game_valid = true;

        for draw in game_draws.split(";") {
            let game_draw = parse_game(draw);
            if !is_draw_valid(&config, game_draw) {
                is_game_valid = false;
                break;
            }
        }
        if is_game_valid {
            println!("{}", game_id);
            valid_games.push(game_id);
        }
    }

    let sum = valid_games.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
}

fn parse_game(game : &str) -> Game {
    let mut game_params = Game {
        red: 0,
        green: 0,
        blue: 0,
    };

    for color_cubes in game.split(",") {
        match color_cubes.trim_start().split_once(' ') {
            Some((draw, "red")) => game_params.red = draw.parse().unwrap(),
            Some((draw, "green")) => game_params.green = draw.parse().unwrap(),
            Some((draw, "blue")) => game_params.blue = draw.parse().unwrap(),
            Some((_, &_)) => (),
            None => (),
        }
    }
    
    return game_params;
}

fn parse_game_id(game_header : &str) -> u32 {
    let ("Game", game_id) = game_header.split_once(" ").unwrap() else { return 0 };
    return game_id.parse().unwrap();
}

fn is_draw_valid(config : &Game, draw : Game) -> bool {
    if config.red >= draw.red && config.green >= draw.green && config.blue >= draw.blue {
        return true;
    } else {
        return false;
    }
}

// games 1, 2, and 5 would have been possible
// total: 8

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn power(game : &Game) -> u32 {
    return game.red * game.green * game.blue;
}

fn print(game: &Game) {
    println!("{} red {} green {} blue", game.red, game.green, game.blue);
}
