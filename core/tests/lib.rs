use hypervariate_core::game::GameMatch;
use rand::Rng;

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
                        variables: Default::default(),
                        time: 0,
                    }
                }
            }
        }

        GameMatch {
            winner: 1,
            variables: Default::default(),
            time: 0,
        }
    }
}

mod tests {
    use crate::RangeGuessingGame;
    use hypervariate_core::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_guessing_game() {
        let mut game = RangeGuessingGame::new(100, 10);
        let result = game.simulate();
        assert!(result.winner == 0 || result.winner == 1);
    }

    #[test]
    fn test_state() {
        let state = State {
            variables: Arc::new(Mutex::new(game::GameVariables::default())),
        };
        assert_eq!(state.variables.lock().unwrap().variables.len(), 0);
    }

    #[test]
    fn test_balance() {}
}
