use std::env::current_dir;

use crate::config::Config;
use crate::step::{App, Step};

#[allow(dead_code)]
pub struct Executor {
    steps: Vec<ExecStep>,
    current_step: Option<String>,
}

impl Executor {
    #[allow(dead_code)]
    pub fn load(config: &Config) -> Executor {
        Executor {
            steps: config.step.iter().map(ExecStep::from).collect(),
            current_step: None,
        }
    }
}

impl From<&Step> for ExecStep {
    fn from(step: &Step) -> Self {
        ExecStep {
            label: String::from(&step.label),
            app: step.app.iter().map(ExecApp::from).collect(),
        }
    }
}

impl From<&App> for ExecApp {
    fn from(app: &App) -> Self {
        ExecApp {
            name: match &app.name {
                Some(name) => String::from(name),
                None => String::from(&app.command),
            },
            command: String::from(&app.command),
            args: app.arguments.clone(),
            current_dir: match &app.cwd {
                Some(path) => String::from(path),
                None => String::from(
                    current_dir()
                        .expect("Could not get working dir")
                        .to_str()
                        .unwrap(),
                ),
            },
        }
    }
}

#[allow(dead_code)]
pub struct ExecApp {
    name: String,
    command: String,
    args: Option<Vec<String>>,
    current_dir: String,
}

#[allow(dead_code)]
pub struct ExecStep {
    label: String,
    app: Vec<ExecApp>,
}
