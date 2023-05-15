use crate::fa::{Acceptor, FAConfig, ReadFAConfig, State};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
pub struct NFA {
    /// Evaluate and validate the DFA described by a given DFAConfig
    states: HashMap<usize, State>,
    initial_state: usize,
    final_states: HashSet<usize>,
    transitions: Vec<Vec<Vec<char>>>, // adjacency matrix for the state graph
}

impl ReadFAConfig for NFA {
    fn from_file(path: PathBuf) -> Self {
        let config = FAConfig::from_file(path);
        Self::from_config(config)
    }

    fn from_config(config: FAConfig) -> Self {
        let epsilon = '\u{03F5}';
        let mut states = HashMap::new();
        let mut state_to_idx: HashMap<String, usize> = HashMap::new();
        let mut initial_state: usize = usize::MAX;

        // can't use the epsilon variable here because it's a char
        // and config.alphabet is a HashSet<String>; .contains expects
        // &String and not a char
        if config.alphabet.contains(&"\u{03F5}".to_owned()) {
            panic!("Error reading configuration: Alphabet cannot contain the symbol \u{03F5} as it is used for null transitions");
        }

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
                // null transition
                let y = state_to_idx.get(&transition.from).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                let x = state_to_idx.get(&transition.to).expect(
                    format!("Error reading configuration: State {} not found in set of all states while parsing transition {}", transition.from, idx + 1).as_str()
                );
                // let the unicode character epsilon denote a null transition
                // NOTE: this means that that character cannot be a part of the alphabet!
                transitions[*y][*x].push(epsilon);
            }
        }

        NFA {
            states,
            initial_state,
            final_states,
            transitions,
        }
    }
}

impl Acceptor for NFA {
    fn test_string(&self, s: String) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        backtrack(self, self.initial_state, 0, &chars)
    }
}

fn backtrack(nfa: &NFA, current_state: usize, current_char: usize, chars: &Vec<char>) -> bool {
    if current_char == chars.len() {
        let a = nfa.final_states.contains(&current_state);
        return a;
    }
    let mut result = false;
    for (idx, neighborhood) in nfa.transitions[current_state].iter().enumerate() {
        if neighborhood.is_empty() {
            continue;
        }
        // I used two separate if statements because if there is a state that can be reached
        // both by consuming the current character and by null transition, BOTH these options
        // need to be explored to see if it reaches an accepting state. Using an `else if` for
        // the second condition would mean that only the first path would be explored (the one by
        // consuming current character).
        if neighborhood.contains(&chars[current_char]) {
            // if ANY of the paths result in an acceptance, the string is accepted
            result |= backtrack(nfa, idx, current_char + 1, chars);
        }
        if neighborhood.contains(&'\u{03F5}') {
            // use the null transition to go to the next state without consuming a character
            result |= backtrack(nfa, idx, current_char, chars);
        }
    }
    result
}
