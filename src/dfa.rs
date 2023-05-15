use crate::fa::{Acceptor, FAConfig, ReadFAConfig, State};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
pub struct DFA {
    /// Evaluate and validate the DFA described by a given DFAConfig
    states: HashMap<usize, State>,
    initial_state: usize,
    final_states: HashSet<usize>,
    transitions: Vec<Vec<Vec<char>>>, // adjacency matrix for the state graph
}

impl ReadFAConfig for DFA {
    fn from_file(path: PathBuf) -> Self {
        let config = FAConfig::from_file(path);
        Self::from_config(config)
    }

    fn from_config(config: FAConfig) -> Self {
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
            if let Some(on) = &transition.on {
                if !config.alphabet.contains(on) {
                    panic!("Error reading configuration: Character {} not present in given alphabet for DFA", on);
                } else {
                    let y = state_to_idx.get(&transition.from).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                    let x = state_to_idx.get(&transition.to).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                    transitions[*y][*x].push(on.chars().next().expect("Error reading configuration: exhausted string iterator while parsing transitions"));
                }
            } else {
                panic!("Error reading configuration: Null transition not allowed in DFAs");
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
