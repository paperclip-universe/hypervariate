use hypervariate_core::game::{GameMatch, GameVariables};
use maplit::hashmap;
use rand::Rng;
use std::collections::HashMap;

struct RangeGuessingGame {
    range_upper: u32,
    attempts: u32,
}

impl RangeGuessingGame {
    pub fn new(range_upper: u32, attempts: u32) -> Self {
        Self {
            range_upper,
            attempts,
        }
    }

    pub fn get_variables(&self) -> HashMap<String, f64> {
        hashmap! {
            "attempts".to_owned() => self.attempts as f64,
            "range_upper".to_owned() => self.range_upper as f64,
        }
    }

    // Simulates playing a higher-lower game with a range and number of attempts.
    // A win for the guesser is when they guess the number correctly, and a win for the chooser is when the guesser runs out of attempts.
    pub fn simulate(&mut self) -> GameMatch {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(0..self.range_upper);
        let mut guess = self.range_upper / 2;

        for _ in 0..self.attempts {
            match guess.cmp(&number) {
                std::cmp::Ordering::Less => guess += (self.range_upper - guess) / 2,
                std::cmp::Ordering::Greater => guess /= 2,
                std::cmp::Ordering::Equal => {
                    return GameMatch {
                        winner: 0,
                        variables: GameVariables {
                            variables: self.get_variables(),
                        },
                        time: 0,
                    }
                }
            }
        }

        GameMatch {
            winner: 1,
            variables: GameVariables {
                variables: self.get_variables(),
            },
            time: 0,
        }
    }
}

mod tests {
    use crate::RangeGuessingGame;
    use hypervariate_core::{game::GameMatches, *};

    #[test]
    fn test_guessing_game() {
        let mut game = RangeGuessingGame::new(100, 10);
        let result = game.simulate();
        assert!(result.winner == 0 || result.winner == 1);
    }

    #[test]
    fn test_state() {
        let state = State {
            variables: game::GameVariables::default(),
            matches: game::GameMatches::default(),
        };
        assert_eq!(state.variables.variables.len(), 0);
    }

    #[test]
    fn test_balance() {
        let mut game = RangeGuessingGame::new(100, 10);
        let mut game2 = RangeGuessingGame::new(10000, 7);
        let mut matches = GameMatches::new(
            2,
            game::GameVariables {
                variables: game.get_variables(),
            },
        );

        let mut results = vec![];

        for _ in 0..=100 {
            results.push(game.simulate());
            results.push(game2.simulate());
        }

        matches.report_results(results);
        let balanced = matches.balance().unwrap();
        println!("{:?}", balanced);
        assert_ne!(
            game::GameVariables {
                variables: game.get_variables(),
            },
            balanced
        );
    }
}
