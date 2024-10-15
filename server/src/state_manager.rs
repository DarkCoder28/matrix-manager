use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::image_manager::HashedImages;

pub(crate) type StateWrapper = Arc<Mutex<State>>;

    #[derive(Default, Debug)]
pub(crate) struct State {
    pub(crate) board_variable_values: HashMap<String, VariableCache>,
    pub(crate) image_hashes: HashedImages,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub(crate) struct VariableCache {
    pub(crate) time_entered: i64,
    pub(crate) value: String,
}