use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use eyre::Result;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    pub num_teams: u32,
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

    // (value, is_winner)
    type SingleVariableTeamResult = (u64, bool);

    // Calculates the correlation between a variable and a team winning/losing.
    // This returns the correlation coefficient, which is a value between -1 and 1.
    pub fn calculate_variable_correlation_single(self, variable: String, team: u32) -> Result<f64> {
        let mut correlations = HashMap::new();

        // For each team, we want to see if the variable correlates with a team winning or losing
        for team in 0..=self.num_teams {
            let mut single_correlation: Vec<SingleVariableTeamResult> = Vec::new();

            for result in &self.results {
                let winning_team = result.winner;
                let variable = result.variables.variables.get(&variable);

                if let Some(variable) = variable {
                    single_correlation.push((
                        *variable as u64,
                        winning_team == team,
                    ));
                }
            }

            // If all results have the same value, we can't calculate a correlation
            let first = single_correlation[0];
            if single_correlation.iter().all(|x| x.0 == first.0) {
                return Err(eyre!("All results have the same value for this variable"));
            }

            // Calculate the correlation coefficient
            
        }

    // TODO: better variable balancing
    /// Balances the variables based on the results of the matches to make the game fairer
    /// It does this by attempting to correlate whether a variable is higher or lower with a team winning or losing
    /// It then adjusts the variable to make it more likely for the losing team to win
    pub fn balance(&mut self, starting_variables: GameVariables) -> GameVariables {
        let mut adjusted_variables = starting_variables.clone();

        // For each variable, we want to see if it correlates with a team winning or losing
        for (variable_name, variable_value) in starting_variables.variables {
            let mut total_winners = 0.0;
            let mut total_losers = 0.0;

            // For each match, we want to see if this variable correlates with a team winning or losing
            for result in &self.results {
                let winning_team = result.winner;
                let variable = result.variables.variables.get(&variable_name);

                // If the variable doesn't exist in the match, we can't use it to balance
                if variable.is_none() {
                    continue;
                }

                // If the variable is higher than the average, it is more likely to win
                if variable.unwrap() > &variable_value {
                    if winning_team == 0 {
                        total_winners += 1.0;
                    } else {
                        total_losers += 1.0;
                    }
                } else if winning_team == 0 {
                    total_losers += 1.0;
                } else {
                    total_winners += 1.0;
                }

                // TODO: Figure out how to adjust variable
                // If the variable is more likely to win and the winners are greater than the losers, we should lower the variable by a number proportional to the difference
                if total_winners > total_losers {
                    adjusted_variables.variables.insert(
                        variable_name.clone(),
                        variable_value * (total_losers / total_winners) / 4f64,
                    );
                } else {
                    adjusted_variables.variables.insert(
                        variable_name.clone(),
                        variable_value * (total_winners / total_losers) / 4f64,
                    );
                }
            }
        }

        adjusted_variables
    }
}
