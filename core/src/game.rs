use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVariables {
    pub variables: HashMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameMatch {
    pub time: u64, // TODO: Is there a dedicated timestamp type?
    pub winner: u64,
    pub variables: GameVariables,
}

fn get_result_distribution_from_vec(vec: Vec<GameMatch>) -> HashMap<u64, i32> {
    let mut distributions = HashMap::new();

    for value in vec {
        let count = distributions.entry(value.winner).or_insert(0);
        *count += 1;
    }

    distributions
}

// TODO: Move to database (just a small todo)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameMatches {
    pub games_played: u64,
    pub results: Vec<GameMatch>,
}

impl GameMatches {
    pub fn report_result(mut self, result: GameMatch) {
        self.games_played += 1;
        self.results.push(result);
    }

    pub fn get_results_after_timestamp(self, timestamp: u64) -> Vec<GameMatch> {
        self.results
            .into_iter()
            .filter(|win| win.time > timestamp)
            .collect()
    }

    pub fn get_result_distribution(self) -> HashMap<u64, i32> {
        get_result_distribution_from_vec(self.results)
    }

    pub fn get_result_distribution_after_timestamp(self, timestamp: u64) -> HashMap<u64, i32> {
        get_result_distribution_from_vec(self.get_results_after_timestamp(timestamp))
    }

    // TODO: better variable balancing
    pub fn balance(&mut self, variables: GameVariables) {
        let result_distribution = self.get_result_distribution();

        let num_games = self.games_played as f64;
    }
}
