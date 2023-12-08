// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
type Play = (Color, i32);

#[derive(Debug, Default)]
struct Game {
    id: i32,
    games: Vec<Play>,
}

#[derive(Debug)]
enum Color {
    Blue,
    Red,
    Green,
}

fn parse_hand(hand: &str) -> Play {
    todo!();
}

fn parse_game(game: &str) -> Vec<Play> {
    let mut games: Vec<Play> = vec![];
    let plays: Vec<&str> = game.split(";").collect();
    for hand in plays {
        todo!();
    }
    todo!();
}

fn parse_gameid(game: &str) -> Result<i32, String> {
    let g: Vec<&str> = game.trim().split(" ").collect();
    if g.len() == 2 {
        g[1].parse::<i32>().map_err(|_| Err(" Wrong Game Id")
    } else {
        Err(" Wrong Game Id")
    }
}

fn parse_row(row: &str) -> Result<Game, String> {
    let game: Vec<&str> = row.split(":").collect();
    if game.len() == 2 {
        let plays: Vec<Play> = parse_game(game[1]);
    } else {
        Result::Err("Row not well formed")
    }
}

fn main() {
    println!("Hello, world!");
}
