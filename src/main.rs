use std::error::Error;
use std::collections::HashSet;
use std::io;
mod game;
use crate::game::Game;
mod wins;
use wins::*;
use crate::game::MyError;
use plotters::prelude::*;
extern crate plotters;

fn team_win_rate(games: &Vec<Game>, team: &String, seasons: &Vec<usize>) -> f64 {
    // for a given team and range of seasons, find the % of games they won based on the number of games they played.
    let mut appearances = 0;
    let mut wins = 0;
    let mut percent = 0.0;
    for season in seasons.iter() {
        for game in games.iter() {
            if *season == game.season {
                // home and away teams are classed differently, so handle with two conditionals.
                if *team == game.home {
                    appearances += 1;
                    if game.result == "H".to_string() {
                        wins += 1;
                    }
                if *team == game.away {
                    appearances += 1;
                    if game.result == "A".to_string() {
                        wins += 1
                    }
                }
                }
            }
        }
        percent = (wins as f64)/(appearances as f64).round();
    }
    percent * 100.0
}

fn top_percent(games: &Vec<Game>, teams: &HashSet<String>, n: usize, seasons: &Vec<usize>) -> Vec<(String, f64)> { 
    // for a given set of teams and seasons, return the top n teams based on win percentage.
    let mut team_pct: Vec<(String, f64)> = Vec::new();
    for team in teams.iter() {
        team_pct.push((team.to_string(), team_win_rate(games, team, seasons))); // call team_win_rate on each team
    }
    team_pct.sort_by(|a: &(String, f64), b: &(String, f64)| b.1.partial_cmp(&a.1).unwrap()); //  win percentage is in a fairly strict range so no problems with lexicographic ordering - just sort by percent 
    let mut top_pct = Vec::new();
    for i in 0..n {
        top_pct.push(team_pct[i].clone());
    }
    top_pct
}

fn team_seasons(games: &Vec<Game>, team: &String, seasons: &Vec<usize>) -> (Vec<usize>, usize) {
    // based on a team and a range of seasons, count all the seasons in that range that they appeared in the league.
    let mut seasons_list = Vec::new();
    for season in seasons.iter() {
        for game in games.iter() {
            if *season == game.season {
                if (*team == game.home) | (*team == game.away) { // if they appeared in any of the games that season
                    if !seasons_list.contains(season) {
                        seasons_list.push(*season);
                    }
                }
            }
        }
    }
    (seasons_list.clone(), seasons_list.len())
}

fn top_appearances(games: &Vec<Game>, teams: &HashSet<String>, seasons: &Vec<usize>, n: usize) -> Vec<(String, usize)> {
    // similar to top_pct - returns the top n teams by seasons appeared in the league.
    let mut team_appearances: Vec<(String, usize)> = Vec::new();
    for team in teams.iter() {
        team_appearances.push((team.to_string(), team_seasons(games, team, seasons).1));
    }
    team_appearances.sort_by(|a: &(String, usize), b: &(String, usize)| b.1.cmp(&a.1));
    let mut top_app = Vec::new();
    for i in 0..n {
        top_app.push(team_appearances[i].clone());
    }
    top_app
}

fn goal_avg(games: &Vec<Game>, season: usize) -> f64 {
    // calculates the average number of goals scored per game in a season.
    let mut goal_total = 0;
    let mut game_count = 0;
    for game in games {
        if season == game.season {
            goal_total += game.home_goals;
            goal_total += game.away_goals;
            game_count += 1;
        }
    }
    return (goal_total as f64)/(game_count as f64);
} 

fn greatest_interval(games: &Vec<Game>, team: &String, seasons: &Vec<usize>) -> Game {
    // calculates the greatest game-winning interval for a given team in a given range of seasons.
    // returns a Game instance with all the information about that game.
    let mut biggest_game = games[0].clone(); // set the biggest game as the first one by default
    let mut goal_differential = 0;
    for game in games {
        for season in seasons {
            if game.season == *season {
                if game.home == *team && game.result == "H" {
                    if game.home_goals - game.away_goals > goal_differential {
                        biggest_game = game.clone();
                        goal_differential = game.home_goals - game.away_goals;
                    }
                }
                if game.away == *team && game.result == "A" {
                    if game.away_goals - game.home_goals > goal_differential {
                        biggest_game = game.clone();
                        goal_differential = game.away_goals - game.home_goals;
                    }
                }
            }
        }
    }
    biggest_game
}

fn user_choice(games: &Vec<Game>, all_seasons: &Vec<usize>, all_teams: &HashSet<String>) -> Result<(), Box<dyn Error>>{
    // takes in three user inputs: a team name, a starting season, and an ending season
    // returns an empty Result enum if successful; prints information about that team and season range.
    let mut team_input = String::new();
    println!("Enter a team: ");
    io::stdin().read_line(&mut team_input).expect("Failed to read line");
    team_input = team_input.trim().to_string();

    if !all_teams.contains(&team_input) {
        println!("Please enter a valid team. Here is the full list of Premier League teams: {:?}", all_teams);
        return Err(Box::new(MyError("You did not enter a valid team name".to_string())));
    }

    let mut season_start_input_string = String::new();
    println!("Enter a starting season from 1993 to 2023: ");
    io::stdin().read_line(&mut season_start_input_string).expect("Failed to read line");
    season_start_input_string = season_start_input_string.trim().to_string();
    let season_start_input = match season_start_input_string.parse::<usize>() { // ensure that input is a valid digit
        Ok(input) => input,
        Err(_) => {
            println!("Enter a valid season in digits.");
            return Err(Box::new(MyError("There was an error".to_string())));
        },
    };
    let mut season_end_input_string = String::new();
    println!("Enter an ending season from 1993 to 2023: ");
    io::stdin().read_line(&mut season_end_input_string).expect("Failed to read line");
    season_end_input_string = season_end_input_string.trim().to_string();
    let season_end_input = match season_end_input_string.parse::<usize>() {
        Ok(input) => input,
        Err(_) => {
            println!("Enter a valid season in digits.");
            return Err(Box::new(MyError("There was an error".to_string())));
        },
    };
    if season_start_input > season_end_input { // handle all possible errors with team 
        println!("Your starting season must be earlier than your ending season.");
        return Err(Box::new(MyError("There was an error".to_string())));
    }
    if !all_seasons.contains(&season_start_input) | !all_seasons.contains(&season_end_input) {
        println!("One or more of your seasons is not in the valid range of 1993 to 2023.");
        return Err(Box::new(MyError("There was an error".to_string())));
    }
    let chosen_seasons: Vec<usize> = (season_start_input..=season_end_input).collect();
    let mut total_games = 0;
    for game in games {
        for season in &chosen_seasons {
            if *season == game.season {
                if team_input == game.home {
                    total_games += 1;
                }
                else if team_input == game.away {
                    total_games += 1;
                }
            }
        }
    }
    if total_games == 0 {
        println!("This team did not play in the Premier League for any of the time you specified.");
        println!("{} played in the PL during the following seasons: {:?}", team_input, team_seasons(games, &team_input, all_seasons).0);
        return Err(Box::new(MyError("There was an error".to_string())));
    }
    let (team_chosen_seasons, team_seasons_count) = team_seasons(games, &team_input, &chosen_seasons);
    println!();
    println!("{} played in {} seasons over that interval: {:?}", team_input, team_seasons_count, team_chosen_seasons);
    println!();
    let goal_int = greatest_interval(&games, &team_input, &chosen_seasons);
    println!("The biggest win interval for {} in those seasons was in the below game:", team_input);
    goal_int.print();
    let win_rate = team_win_rate(&games, &team_input, &chosen_seasons);
    println!("The {} win rate for the {} to {} seasons is {:.4}%.", team_input, chosen_seasons[0], chosen_seasons.last().unwrap(), win_rate);
    return Ok(());
}

fn main() {
    let mut df = game::DataFrame::new();
    let games = df.read_csv("pl_matches.csv").unwrap();
    let all_seasons: Vec<usize> = (1993..=2023).collect();
    let mut all_teams = HashSet::new();
    for game in games.iter() {
        all_teams.insert(game.home.clone());
    }
    let rankings = 10; // get the top 10 in both success categories
    println!("Over {} seasons, a total of {} teams have competed in the Premier League.", all_seasons.len(), all_teams.len());
    let top10_pct = top_percent(&games, &all_teams, rankings, &all_seasons);
    println!();
    println!("Most successful teams by win percentage:");
    for (i, (team, pct)) in top10_pct.iter().enumerate() {
        println!("{}: {} with a win percentage of {:.4}", (i+1), team, pct)
    }
    println!();
    println!("Most successful teams by number of seasons:");
    let top10_app = top_appearances(&games, &all_teams, &all_seasons, rankings);
    for (i, (team, season_apps)) in top10_app.iter().enumerate() {
        println!("{}: {} with {} total seasons in the PL", (i+1), team, season_apps)
    }

    let mut home_advantages: Vec<(usize, f64)> = Vec::new();
    let mut avg_home = 0.0;
    let mut avg_away = 0.0;
    for szn in &all_seasons {
        home_advantages.push((*szn, home_pct(&games, *szn))); // collect each season's average home win rate
        avg_home += home_pct(&games, *szn);
        avg_away += away_pct(&games, *szn);
    }

    avg_home = avg_home/(all_seasons.len() as f64);
    avg_away = avg_away/(all_seasons.len() as f64);
    let avg_diff = avg_home - avg_away;
    let mut worst_szn = 0;
    let mut worst_adv = 100.0;
    let mut second_worst_szn = 0;
    let mut diff = 0.0;
    for (szn, adv) in &home_advantages {
        if *adv < worst_adv {
            diff = worst_adv - adv;
            worst_adv = *adv;
            second_worst_szn = worst_szn;
            worst_szn = *szn // season with least amount of home advantage (difference between home and away win rate)
        }
    }
    let mut x_values: [f64; 31] = [1993.0; 31];
    for i in 0..31 {
        x_values[i] += i as f64;
    } // update array in place so each x value is a season
    let drawing_area = BitMapBackend::new("all_time_rates.png", (640, 480)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Home, Away, and Draw results - 1993-2023", ("sans-serif", 20).into_font())
        .margin(10).set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d(1993.0..2023.0, 10.0..60.0).unwrap();
    chart_builder.configure_mesh().draw().unwrap();
    chart_builder.draw_series(LineSeries::new(x_values.map(|x | (x, home_pct(&games, x as usize))), BLACK)).unwrap() // for each season, compute the home win rate
        .label("Home win rate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK)); // add legend
    chart_builder.draw_series(LineSeries::new(x_values.map(|x | (x, away_pct(&games, x as usize))), RED)).unwrap()
        .label("Away win rate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart_builder.draw_series(LineSeries::new(x_values.map(|x | (x, draw_pct(&games, x as usize))), BLUE)).unwrap()
        .label("Draw rate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart_builder.configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    println!();
    println!("The average home win-rate in the Premier League across all seasons is {:.3}%, compared to an away win-rate of {:.3}%.", avg_home, avg_away);
    println!("The season with the lowest home-win rate was {} with a home-win rate of {:.3}%, which is {:.3}% worse than the second-worst season of {}.", worst_szn, worst_adv, diff, second_worst_szn);
    let this_draw = draw_pct(&games, worst_szn.clone());
    let this_away = away_pct(&games, worst_szn.clone());
    println!("In {}, the draw rate was {:.3}% and the away-win rate was {:.3}%.", worst_szn, this_draw, this_away);
    println!("This is a home-away differential of {:.3}%. The average home-away differential across all {} seasons is {:.3}%.", worst_adv - this_away, all_seasons.len(), avg_diff);
    let mut all_goal_avg = 0.0;
    let mut goal_averages = Vec::new();
    for szn in &all_seasons {
        all_goal_avg += goal_avg(&games, *szn);
        goal_averages.push((*szn, goal_avg(&games, *szn)));
    }
    all_goal_avg = all_goal_avg/(all_seasons.len() as f64);
    println!("The average number of goals scored in a PL game is {:.4}.", all_goal_avg);
    let mut most_szn: usize = 0;
    let mut most_goals = 0.0;
    for (szn, goals) in &goal_averages {
        if *goals > most_goals {
            most_szn = *szn;
            most_goals = *goals;
        }
    }
    let drawing_area = BitMapBackend::new("goal_averages.png", (640, 480)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Total goals per season - 1993-2023", ("sans-serif", 20).into_font())
        .margin(10).set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d(1993.0..2023.0, 2.0..3.0).unwrap();
    chart_builder.configure_mesh().draw().unwrap();
    chart_builder.draw_series(LineSeries::new(x_values.map(|x | (x, goal_avg(&games, x as usize))), BLACK)).unwrap()
        .label("Average goals per game")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    chart_builder.configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    println!("The season with the most average goals per game was {} with {:.4} goals per game.", most_szn, most_goals);
    println!();
    let _ = user_choice(&games, &all_seasons, &all_teams);
}

#[test]
fn test_result_rates() {
    let mut df = game::DataFrame::new();
    let games = df.read_csv("pl_matches.csv").unwrap();
    let season = 2023;
    let mut total_rate = home_pct(&games, season.clone());
    total_rate += draw_pct(&games, season.clone());
    total_rate += away_pct(&games, season.clone());
    assert_eq!(total_rate, 100.0, "The total results percentage should be 100% for any given season!");
}

#[test]
fn test_team_appearances() {
    let mut df = game::DataFrame::new();
    let games = df.read_csv("pl_matches.csv").unwrap();
    let manutd = String::from("Manchester Utd");
    let seasons: Vec<usize> = (1993..=2023).collect();
    let manutd_seasons = team_seasons(&games, &manutd, &seasons).1;
    assert_eq!(manutd_seasons, seasons.len(), "Manchester Utd has played in every season of the Premier League!");
}