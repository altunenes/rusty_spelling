use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub date_time: String,
    pub score: u32,
    pub correct_words: Vec<String>,
    pub incorrect_words: Vec<String>,
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
        let file = File::create(Path::new(file_path))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    pub fn load_results(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(file_path))?;
        let reader = BufReader::new(file);
        let results = serde_json::from_reader(reader)?;
        Ok(results)
    }
}