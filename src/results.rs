use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub date_time: String,
    pub score: u32,
    pub correct_words: Vec<String>,
    pub incorrect_words: Vec<String>,
    pub wrong_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    pub game_results: Vec<GameResult>,
}

impl Results {
    pub fn new() -> Self {
        Results {
            game_results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: GameResult) {
        self.game_results.push(result);

    }

    pub fn save_results(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
        writeln!(file, "{}", serde_json::to_string(self).unwrap())?;
        Ok(())
    }

    //* this for further development
    
    //pub fn load_results(file_path: &str) -> Result<Self, std::io::Error> {
       // let file = File::open(Path::new(file_path))?;
       // let reader = BufReader::new(file);
       //let results = serde_json::from_reader(reader)?;
       // Ok(results)
    //}
}