use crate::math::calculate_correlation_coefficient;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct GameVariables {
    pub variables: HashMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct GameMatch {
    pub time: u64, // TODO: Is there a dedicated timestamp type?
    pub winner: u64,
    pub variables: GameVariables,
}

// TODO: cleanup cleanup everybody everywhere
fn get_result_distribution_from_vec(vec: Vec<GameMatch>) -> HashMap<u64, f64> {
    let mut distributions = HashMap::new();
    let mut total = 0;

    for value in vec {
        let count = distributions.entry(value.winner).or_insert(0);
        *count += 1;
        total += 1;
    }

    distributions
        .iter()
        .map(|(x, y)| (*x, *y as f64 / total as f64))
        .collect()
}

// TODO: Move to database (just a small todo)
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct GameMatches {
    pub games_played: u64,
    pub results: Vec<GameMatch>,
    pub num_teams: u32,
    pub base_variables: GameVariables,
}

/// (value, is_winner)
type SingleVariableTeamResult = (u64, bool);

impl GameMatches {
    pub fn new(num_teams: u32, base_variables: GameVariables) -> Self {
        Self {
            games_played: 0,
            results: vec![],
            num_teams,
            base_variables,
        }
    }

    pub fn report_result(mut self, result: GameMatch) {
        self.games_played += 1;
        self.results.push(result);
    }

    pub fn report_results(&mut self, results: Vec<GameMatch>) {
        for result in results {
            self.games_played += 1;
            self.results.push(result);
        }
    }

    pub fn get_results_after_timestamp(self, timestamp: u64) -> Vec<GameMatch> {
        self.results
            .into_iter()
            .filter(|win| win.time > timestamp)
            .collect()
    }

    pub fn get_result_distribution(self) -> HashMap<u64, f64> {
        get_result_distribution_from_vec(self.results)
    }

    pub fn get_result_distribution_after_timestamp(self, timestamp: u64) -> HashMap<u64, f64> {
        get_result_distribution_from_vec(self.get_results_after_timestamp(timestamp))
    }

    /// Calculates the correlation between a variable and a team winning/losing.
    /// This returns the correlation coefficient, which is a value between -1 and 1.
    pub fn calculate_variable_correlation_single_team(
        &self,
        variable: String,
        team: u32,
    ) -> Result<f64> {
        // For each team, we want to see if the variable correlates with a team winning or losing
        let mut single_correlation: Vec<SingleVariableTeamResult> = Vec::new();

        for result in &self.results {
            let winning_team = result.winner;
            let variable = result.variables.variables.get(&variable);

            if let Some(variable) = variable {
                single_correlation.push((*variable as u64, winning_team == team.into()));
            }
        }

        // If all results have the same value, we can't calculate a correlation
        let first = single_correlation[0];
        if single_correlation.iter().all(|x| x.0 == first.0) {
            return Err(eyre!("All results have the same value for this variable"));
        }

        // Calculate the correlation coefficient
        let coefficient = calculate_correlation_coefficient(
            single_correlation
                .iter()
                .map(|corr| (corr.0, corr.1 as u64))
                .collect(),
        );

        return Ok(coefficient);
    }

    pub fn calculate_variable_correlation_single_variable(
        &self,
        variable: String,
    ) -> Result<HashMap<u32, f64>> {
        let mut correlations: HashMap<u32, f64> = HashMap::new();

        for team in 0..self.num_teams {
            correlations.insert(
                team,
                // TODO: fix memory management
                self.calculate_variable_correlation_single_team(variable.clone(), team)?,
            );
        }

        Ok(correlations)
    }

    pub fn calculate_variable_correlation(self) -> Result<HashMap<String, HashMap<u32, f64>>> {
        let mut correlations: HashMap<String, HashMap<u32, f64>> = HashMap::new();

        for variable in &self.base_variables.variables {
            correlations.insert(
                variable.0.to_string(),
                self.calculate_variable_correlation_single_variable(variable.0.to_string())?,
            );
        }

        Ok(correlations)
    }

    // TODO: better variable balancing
    /// Balances the variables based on the results of the matches to make the game fairer
    /// It does this by attempting to correlate whether a variable is higher or lower with a team winning or losing
    /// It then adjusts the variable to make it more likely for the losing team to win
    pub fn balance(self) -> Result<GameVariables> {
        let mut balanced_variables = self.base_variables.clone();
        // TODO: Fix clone() - I hate memory management :P
        let distribution: HashMap<u64, f64> = self.clone().get_result_distribution();
        let correlations: HashMap<String, HashMap<u32, f64>> =
            self.calculate_variable_correlation()?;

        // What should the result distribution be?
        let optimal_distribution: f64 = 1.0 / (distribution.len() as f64);

        // How far off is it?
        let distribution_difference: Vec<(u64, f64)> = distribution
            .iter()
            .map(|(team, amount)| (*team, optimal_distribution - *amount))
            .collect();

        let mut adjustments: HashMap<String, f64> = HashMap::new();

        // Adjust variables based on correlation and distribution difference
        for (variable, correlation_map) in &correlations {
            let mut total_adjustment: f64 = 0.0;

            for (team, correlation) in correlation_map {
                let (team_index, _) = distribution_difference
                    .iter()
                    .find(|(index, _)| *index == *team as u64)
                    .ok_or(eyre!("Invalid index for distribution_difference"))?;

                let (_, difference) = distribution_difference
                    .get(*team_index as usize)
                    .ok_or(eyre!("Invalid index for distribution_difference"))?;

                let team_adjustment = difference * correlation;
                total_adjustment += team_adjustment;
            }

            adjustments.insert(variable.clone(), total_adjustment);
        }

        // Apply adjustments to the variables
        for (variable, adjustment) in &adjustments {
            if let Some(value) = balanced_variables.variables.get_mut(variable) {
                *value += *adjustment;
            }
        }

        Ok(balanced_variables)
    }
}
