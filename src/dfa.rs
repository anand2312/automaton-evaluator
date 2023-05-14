use crate::traits::Acceptor;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct DFAConfig {
    states: Vec<String>,
    initial_state: String,
    final_states: Vec<String>,
    alphabet: HashSet<String>,
    transitions: Vec<Transition>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transition {
    from: String,
    to: String,
    on: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    name: String,
    index: usize,
}

#[derive(Debug)]
pub struct DFA {
    /// Evaluate and validate the DFA described by a given DFAConfig
    states: HashMap<usize, State>,
    initial_state: usize,
    final_states: HashSet<usize>,
    transitions: Vec<Vec<Vec<char>>>, // adjacency matrix for the state graph
}

impl DFAConfig {
    pub fn from_file(path: PathBuf) -> Self {
        let s = fs::read_to_string(path).unwrap();
        let parsed: DFAConfig = serde_json::from_str(s.as_str()).unwrap();
        parsed
    }
}

impl DFA {
    pub fn from_file(path: PathBuf) -> Self {
        let config = DFAConfig::from_file(path);
        Self::from_config(config)
    }

    fn from_config(config: DFAConfig) -> Self {
        let mut states = HashMap::new();
        let mut state_to_idx: HashMap<String, usize> = HashMap::new();
        let mut initial_state: usize = usize::MAX;

        for (idx, name) in config.states.into_iter().enumerate() {
            states.insert(
                idx,
                State {
                    name: name.clone(),
                    index: idx,
                },
            );
            state_to_idx.insert(name.clone(), idx);
            if name == config.initial_state {
                initial_state = idx;
            }
        }

        if initial_state == usize::MAX {
            panic!("Error reading configuration: Initial state not present in set of all states");
        }

        let mut final_states = HashSet::new();
        for name in config.final_states.into_iter() {
            if let Some(s) = state_to_idx.get(&name) {
                final_states.insert(s.to_owned());
            } else {
                panic!(
                    "Error reading configuration: Final state {} not present in set of all states",
                    name
                );
            }
        }

        let mut transitions = vec![vec![vec![]; states.len()]; states.len()];
        for (idx, transition) in config.transitions.into_iter().enumerate() {
            if !config.alphabet.contains(&transition.on) {
                panic!("Error reading configuration: Character {} not present in given alphabet for DFA", transition.on);
            } else {
                let y = state_to_idx.get(&transition.from).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                let x = state_to_idx.get(&transition.to).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                transitions[*y][*x].push(transition.on.chars().next().expect("Error reading configuration: exhausted string iterator while parsing transitions"));
            }
        }

        return DFA {
            states,
            initial_state,
            final_states,
            transitions,
        };
    }
}

impl Acceptor for DFA {
    fn test_string(&self, s: String) -> bool {
        let mut current_state = self.initial_state;
        for c in s.chars() {
            for (idx, neighborhood) in self.transitions[current_state].iter().enumerate() {
                if neighborhood.is_empty() {
                    continue;
                }
                if neighborhood.contains(&c) {
                    current_state = idx;
                }
            }
        }
        self.final_states.contains(&current_state)
    }
}
