use crate::step::Step;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config<'a> {
    #[serde(borrow)]
    pub config_version: Cow<'a, str>,
    #[serde(borrow)]
    pub step: Vec<Step<'a>>,
}

impl Default for Config<'static> {
    fn default() -> Config<'static> {
        Config {
            config_version: Cow::from("0.0.1"),
            step: vec![],
        }
    }
}
