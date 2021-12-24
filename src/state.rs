use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<State>>;

#[derive(Default)]
pub struct State {}
