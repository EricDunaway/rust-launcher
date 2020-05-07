use std::borrow::Cow;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::step::Step;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config<'input> {
    #[serde(borrow)]
    pub config_version: Cow<'input, str>,
    pub step: Vec<Step<'input>>,
}

impl Default for Config<'static> {
    fn default() -> Config<'static> {
        Config {
            config_version: Cow::from("0.0.1"),
            step: vec![],
        }
    }
}
