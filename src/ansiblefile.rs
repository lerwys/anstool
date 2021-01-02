mod module;
mod task;

use self::task::Task;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnsibleFile {
    Tasks(Vec<Task>),
    Object(Box<AnsibleObject>),
}

type AnsibleObject = HashMap<String, AnsibleFile>;

#[derive(Error, Debug)]
pub enum FromParseError {
    #[error("Failed to perform I/O operation")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("Failed to deserialize")]
    DeserializationError {
        #[from]
        source: serde_yaml::Error,
    },
}

impl AnsibleFile {
    /// Implements deserialization of an ansible file from
    /// a Path.
    pub fn from_file(path: &Path) -> Result<Self, FromParseError> {
        // FromParseError implements:
        // impl From<io::Error> for FromParseError
        // so "?" works by calling:
        // Into<FromParseError>::into
        // that is automatially provided when implementing From
        let file = File::open(path)?;
        let ansiblefile = serde_yaml::from_reader(BufReader::new(file))?;

        Ok(ansiblefile)
    }

    #[allow(dead_code)]
    /// Implements deserialization of an ansible file from
    /// a &str.
    pub fn from_str(s: &str) -> Result<Self, FromParseError> {
        let ansiblefile = serde_yaml::from_str(s)?;

        Ok(ansiblefile)
    }
}

#[cfg(test)]
mod tests {
    use super::AnsibleFile;

    #[test]
    fn test_single_task_deserialize() {
        let s = r#"
---
- name: Test task
  module_name:
    test: "Test string"
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_include_vars_deserialize() {
        let s = r#"
---
- name: Test task
  include_vars: file.yml
  module_name:
    test: "Test string"
  when: test is not defined
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_include_tasks_deserialize() {
        let s = r#"
---
- name: Test task
  include_tasks: file.yml
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_include_tasks_single_when_deserialize() {
        let s = r#"
---
- name: Test task
  include_tasks: file.yml
  when: condition is not true
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_include_tasks_multiple_when_deserialize() {
        let s = r#"
---
- name: Test task
  include_tasks: file.yml
  when:
    - condition_1 is not true
    - condition_2 is not true
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_single_when_deserialize() {
        let s = r#"
---
- name: Test task
  module_name:
    test: "Test string"
  when: test is not defined
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_single_task_multiple_when_deserialize() {
        let s = r#"
---
- name: Test task
  module_name:
    test: "Test string"
  when:
    - test is not defined
    - condition 2 is not true
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }

    #[test]
    fn test_multiple_tasks_deserialize() {
        let s = r#"
---
- name: test task 1
  module_name:
    test_1: "test string 1"
  when: test_1 is not defined

- name: test task 2
  module_name:
    test_2: "test string 2"
  when: test_2 is not defined
"#;
        let _ = AnsibleFile::from_str(s).unwrap();
    }
}
