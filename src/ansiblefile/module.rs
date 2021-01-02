use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Module {
    #[serde(flatten)]
    root: Box<ModuleRoot>,
}

type ModuleRoot = HashMap<String, ModuleValues>;

#[derive(Debug, Deserialize)]
pub struct ModuleValues {
    #[serde(flatten)]
    values: Box<Values>,
}

type Values = HashMap<String, String>;
