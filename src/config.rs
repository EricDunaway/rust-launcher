use crate::step::Step;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub config_version: String,
    pub step: Vec<Step>,
}

impl Config {}

impl Default for Config {
    fn default() -> Config {
        Config {
            config_version: String::from("0.0.1"),
            step: vec![],
        }
    }
}
