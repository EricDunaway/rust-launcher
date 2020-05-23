use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App<'a> {
    #[serde(default)]
    #[serde(borrow)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default)]
    #[serde(borrow)]
    pub(crate) cwd: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub(crate) command: Cow<'a, str>,
    #[serde(default)]
    #[serde(borrow)]
    pub(crate) arguments: Option<Vec<Cow<'a, str>>>,
    #[serde(default)]
    #[serde(borrow)]
    pub(crate) trigger: Option<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Step<'a> {
    #[serde(borrow)]
    pub(crate) label: Cow<'a, str>,
    #[serde(borrow)]
    pub(crate) app: Vec<App<'a>>,
}
