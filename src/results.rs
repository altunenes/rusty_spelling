use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct GameResult {
    pub date_time: String,
    pub score: u32,
    pub correct_words: Vec<String>,
    pub incorrect_words: Vec<String>,
    pub wrong_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
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
        let file = Path::new(file_path);
        let mut results = if file.exists() {
            let mut file = File::open(file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str::<Results>(&contents).unwrap_or_else(|_| Results::new())
        } else {
            Results::new()
        };

        results.game_results.extend(self.game_results.clone());

        std::fs::write(file_path, serde_json::to_string(&results).unwrap())?;
        Ok(())
    }
}