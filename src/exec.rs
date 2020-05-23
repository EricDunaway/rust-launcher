use std::borrow::Borrow;
use std::env::current_dir;

use tokio::task;

use crate::config::Config;
use crate::step::{App, Step};

#[allow(dead_code)]
pub struct Executor {
    pub steps: Vec<ExecStep>,
    pub current_step: Option<usize>,
    pub tasks: Vec<task::JoinHandle<()>>,
}

impl Executor {
    pub fn load(config: &Config<'_>) -> Executor {
        Executor {
            steps: config.step.iter().map(ExecStep::from).collect(),
            current_step: None,
            tasks: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<&String> {
        if self.current_step.is_none() {
            self.current_step = Some(0);
        } else if self.current_step.unwrap() < self.steps.len() {
            self.current_step = Some(self.current_step.unwrap() + 1);
        } else {
            return None;
        }

        let task = task::spawn(async move {});

        self.tasks.push(task);

        Some(self.steps[self.current_step.unwrap()].label.borrow())
    }
}

impl From<&Step<'_>> for ExecStep {
    fn from(step: &Step<'_>) -> Self {
        ExecStep {
            label: String::from(step.label.as_ref()),
            app: step
                .app
                .iter()
                .map(|x| -> ExecApp { ExecApp::from(x.borrow()) })
                .collect(),
        }
    }
}

impl From<&App<'_>> for ExecApp {
    fn from(app: &App<'_>) -> Self {
        ExecApp {
            name: match &app.name {
                Some(name) => String::from(name.as_ref()),
                None => String::from(app.command.as_ref()),
            },
            command: String::from(app.command.as_ref()),
            args: match &app.arguments {
                Some(arg) => Some(
                    arg.iter()
                        .map(|x| -> String { String::from(x.as_ref()) })
                        .collect(),
                ),
                None => None,
            },
            current_dir: match &app.cwd {
                Some(path) => String::from(path.as_ref()),
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
    pub name: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub current_dir: String,
}

#[allow(dead_code)]
pub struct ExecStep {
    pub label: String,
    pub app: Vec<ExecApp>,
}
