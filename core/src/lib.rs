use std::sync::{Arc, Mutex};

pub mod game;
pub mod math;

pub fn wrap<T>(item: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(item))
}
pub struct State {
    // pub variables: Arc<Mutex<game::GameVariables>>,
    // pub matches: Arc<Mutex<game::GameMatches>>,
    pub variables: game::GameVariables,
    pub matches: game::GameMatches,
}
