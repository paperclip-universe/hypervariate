use std::sync::{Arc, Mutex};

pub mod game;

pub struct State {
    pub variables: Arc<Mutex<game::GameVariables>>,
}
