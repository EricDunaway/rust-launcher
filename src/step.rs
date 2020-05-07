use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct App<'input> {
    #[serde(borrow)]
    pub(crate) cwd: Cow<'input, str>,
    #[serde(borrow)]
    pub(crate) command: Cow<'input, str>,
    #[serde(borrow)]
    pub(crate) arguments: Vec<Cow<'input, str>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Step<'input> {
    #[serde(borrow)]
    pub(crate) label: Cow<'input, str>,
    #[serde(borrow)]
    pub(crate) app: Vec<App<'input>>,
}
