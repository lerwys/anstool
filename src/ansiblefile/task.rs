use super::module::Module;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    name: Option<String>,
    include_vars: Option<String>,
    include_tasks: Option<String>,
    #[serde(flatten)]
    module: Box<Module>,
    when: Option<WhenOpts>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WhenOpts {
    Single(String),
    Multiple(Vec<String>),
}
