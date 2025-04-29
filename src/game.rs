use std::error::Error;
use std::fmt;

// a module to read the csv and split it into the necessary Structs to analyze each game individually.

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    // the same error handling functions as the dataframe from Hw8.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

pub struct DataFrame {
    // a struct that holds information about the whole csv, representing all match results.
    // used to keep information about the headers and rows consistent and store all the games in one record.
    headers: Vec<String>,
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Game {
    // an individual record of each game with the information accompanying it from the csv.
    pub season: usize,
    week: usize,
    date: String,
    pub home: String,
    pub away: String,
    pub home_goals: usize,
    pub away_goals: usize,
    pub result: String,
}
impl DataFrame {
    pub fn new() -> Self {
        // creates a new empty instance of the DataFrame struct.
        DataFrame {
            headers: Vec::new(),
            games: Vec::new(),
        }
    }
    pub fn read_csv(&mut self, path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
        // reads from the csv. intiializes an individual vector for each game "field"
        // uses a match statement to push info from each column to the correct vector, then collects those into individual games at the end
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .flexible(true)
            .from_path(path)?;
        let mut seasons = Vec::new();
        let mut weeks = Vec::new();
        let mut dates = Vec::new();
        let mut homes = Vec::new();
        let mut aways = Vec::new();
        let mut home_scores = Vec::new();
        let mut away_scores = Vec::new();
        let mut ftrs = Vec::new();
        for h in rdr.headers()? {
            self.headers.push((*h).to_string());
        }
        for result in rdr.records() {
            let r = result.unwrap();
            for (i, elem) in r.iter().enumerate() {
                match i { // each column will be used, so just match the column based on order and push the info from that row to the correct vector
                    0 => seasons.push(elem.parse::<usize>().unwrap()),
                    1 => weeks.push(elem.parse::<usize>().unwrap()),
                    2 => dates.push(elem.to_string()),
                    3 => homes.push(elem.to_string()),
                    4 => home_scores.push(elem.parse::<usize>().unwrap()),
                    5 => away_scores.push(elem.parse::<usize>().unwrap()),
                    6 => aways.push(elem.to_string()),
                    7 => ftrs.push(elem.to_string()),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
        }
        for (i, elem) in homes.iter().enumerate() {
            // every vector is the same length, so just loop through the length and create a new Game instance for each one
            // then push that Game to the games field of the dataframe to keep a record of all games
            let game = Game::new(seasons[i], weeks[i], dates[i].clone(), elem.to_string(), aways[i].clone(), home_scores[i], away_scores[i], ftrs[i].clone());
            self.games.push(game)
        }
        Ok(self.games.clone())
    }
}

impl Game {
    fn new(season: usize, week: usize, date: String, home: String, away: String, home_goals: usize, away_goals: usize, result: String) -> Self {
        // a constructor method to create a new Game instance.
        Game {
            season,
            week,
            date,
            home,
            away,
            home_goals,
            away_goals,
            result,
        }
    }

    pub fn print(&self) {
        // to print the info from each game more elegantly, with more information based on the result.
        if self.result == "H".to_string() {
            println!("{} beat {} {}-{} in a home win.", self.home, self.away, self.home_goals, self.away_goals);
        }
        if self.result == "A".to_string() {
            println!("{} beat {} {}-{} in an away win.", self.away, self.home, self.away_goals, self.home_goals);
        }
        if self.result == "D".to_string() {
            println!("{} played {} in a {}-{} draw.", self.home, self.away, self.home_goals, self.away_goals);
        }
        println!("This game happened on {} during week {} of the {} season.", self.date, self.week, self.season);
        println!();
    }
}