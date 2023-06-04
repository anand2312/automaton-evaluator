#![cfg(feature = "build-binary")]
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct FAConfig {
    /// Describes a finite state automaton.
    pub states: Vec<String>,
    pub initial_state: String,
    pub final_states: Vec<String>,
    pub alphabet: HashSet<String>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub on: Option<String>,
}

impl FAConfig {
    pub fn from_file(path: PathBuf) -> Self {
        let s = fs::read_to_string(path).unwrap();
        let parsed: FAConfig = serde_json::from_str(s.as_str()).unwrap();
        parsed
    }
}

pub trait ReadFAConfig {
    fn from_config(config: FAConfig) -> Self;
    fn from_file(path: PathBuf) -> Self
    where
        Self: Sized,
    {
        let config = FAConfig::from_file(path);
        Self::from_config(config)
    }
}
