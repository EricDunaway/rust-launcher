use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub(crate) cwd: Option<String>,
    pub(crate) command: String,
    #[serde(default)]
    pub(crate) arguments: Option<Vec<String>>,
    pub(crate) trigger: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Step {
    pub(crate) label: String,
    pub(crate) app: Vec<App>,
}
