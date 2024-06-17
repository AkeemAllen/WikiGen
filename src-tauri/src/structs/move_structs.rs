use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Moves {
    pub moves: HashMap<String, MoveDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveDetails {
    pub accuracy: Option<u32>,
    pub pp: u32,
    pub power: Option<u32>,
    #[serde(rename = "type")]
    pub _type: String,
    pub damage_class: String,
    pub past_values: Vec<Value>,
    pub machine_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveSetMove {
    pub learn_method_detail: LearnMethodDetail,
    pub power: Option<u32>,
    pub _type: String,
    pub accuracy: Option<u32>,
    pub pp: u32,
    pub damage_class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LearnMethodDetail {
    LevelLearned(u32),
    MachineName(String),
}

impl fmt::Display for LearnMethodDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
