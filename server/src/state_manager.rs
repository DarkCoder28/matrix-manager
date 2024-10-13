use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub(crate) type StateWrapper = Arc<Mutex<State>>;

pub(crate) struct State {
    pub(crate) board_variable_values: HashMap<String, VariableCache>
}

impl State {
    pub(crate) fn new() -> State {
        State {
            board_variable_values: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VariableCache {
    pub(crate) time_entered: i64,
    pub(crate) value: String,
}