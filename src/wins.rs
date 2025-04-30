use crate::Game;

// a module containing functions that are used to compute result rates in a given season.

pub fn home_pct(games: &Vec<Game>, season: usize) -> f64 {
    // calculates the percentage of games in a season where the home team won.
    let mut home_win = 0;
    let mut game_count = 0;
    for game in games {
        if season == game.season {
            if game.result == "H" {
                home_win += 1;
            }
            game_count += 1;
        }
    }
    return (home_win as f64)/(game_count as f64) * 100.0;
}

pub fn draw_pct(games: &Vec<Game>, season: usize) -> f64 {
    // calculates the percentage of games in a season that ended in a draw.
    let mut draw = 0;
    let mut game_count = 0;
    for game in games {
        if season == game.season {
            if game.result == "D" {
                draw += 1;
            }
            game_count += 1;
        }
    }
    return (draw as f64)/(game_count as f64) * 100.0;
}

pub fn away_pct(games: &Vec<Game>, season: usize) -> f64 {
    // calculates the percentage of games in a season where the away team won.
    let mut away_win = 0;
    let mut game_count = 0;
    for game in games {
        if season == game.season {
            if game.result == "A" {
                away_win += 1;
            }
            game_count += 1;
        }
    }
    return (away_win as f64)/(game_count as f64) * 100.0;
}