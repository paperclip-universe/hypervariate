use std::sync::{Arc, Mutex};

pub mod game;

pub struct State {
    variables: Arc<Mutex<game::GameVariables>>,
    
}
