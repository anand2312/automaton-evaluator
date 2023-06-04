use crate::traits::Acceptor;
use std::collections::{HashMap, HashSet};
#[cfg(feature="build-binary")]
use crate::config::*;

#[derive(Debug)]
pub struct NFA {
    /// Evaluate and validate the DFA described by a given DFAConfig
    pub states: HashMap<usize, String>,
    pub initial_state: usize,
    pub final_states: HashSet<usize>,
    pub transitions: Vec<Vec<Vec<char>>>, // adjacency matrix for the state graph
}

impl NFA {
    pub fn new(states: HashMap<usize, String>, initial_state: usize, final_states: HashSet<usize>, transitions: Vec<Vec<Vec<char>>>) -> Self {
        Self {
            states,
            initial_state,
            final_states,
            transitions
        }
    }
}
#[cfg(feature="build-binary")]
impl ReadFAConfig for NFA {
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
                name.clone(),
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
        acceptor(self, s)
    }
}

fn _backtrack(nfa: &NFA, current_state: usize, current_char: usize, chars: &Vec<char>) -> bool {
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
            result |= _backtrack(nfa, idx, current_char + 1, chars);
        }
        if neighborhood.contains(&'\u{03F5}') {
            // use the null transition to go to the next state without consuming a character
            result |= _backtrack(nfa, idx, current_char, chars);
        }
    }
    result
}

// Another implementation for NFA evaluation: keep track of the set of possible current states
// after consuming each character.
// 0. Find the null closure of the current state(s)
// 1. When the NFA can go to multiple states on a single input, keep track of all of these states in a set
// 2. Find the null closure of the set of states after step 0 and 1
// After consuming all characters, if the set of current states and set of final states have non-empty
// intersection, the string was accepted.
// This approach should run in expected linear time, as opposed to the backtracking approach taking
// possibly exponential time as the number of paths through the graph is 2^n
// The backtracking approach only needs to be used if we needed the exact PATH taken through the graph.
fn null_closure(nfa: &NFA, current_states: HashSet<usize>) -> HashSet<usize> {
    let mut res = current_states.clone();
    let mut old_length = current_states.len();
    // println!("before finding closure = {:?}", &current_states);
    for state in current_states {
        for (idx, neighborhood) in nfa.transitions[state].iter().enumerate() {
            if neighborhood.contains(&'\u{03F5}') {
                res.insert(idx);
            }
        }
    }

    let mut new_length = res.len();
    if old_length == new_length {
        return res;
    }
    let mut next_closure = null_closure(nfa, res);
    while new_length != old_length {
        old_length = next_closure.len();
        next_closure = null_closure(nfa, next_closure);
        new_length = next_closure.len();
    }
    // println!("null closure = {:?}", &next_closure);
    next_closure
}

fn current_states_after_consuming_char(nfa: &NFA, current_states: HashSet<usize>, current_char: char) -> HashSet<usize> {
    let closure = null_closure(nfa, current_states);
    let mut next_states = HashSet::new();
    for state in closure {
        for (idx, neighborhood) in nfa.transitions[state].iter().enumerate() {
            if neighborhood.contains(&current_char) {
                next_states.insert(idx);
            }
        }
    }
    null_closure(nfa, next_states)
}

fn acceptor(nfa: &NFA, s: String) -> bool {
    let mut current_states = HashSet::new();
    current_states.insert(nfa.initial_state);
    // println!("initial state = {:?}", &current_states);
    for character in s.chars() {
        current_states = current_states_after_consuming_char(nfa, current_states, character);
    }
    !current_states.is_disjoint(&nfa.final_states) // if the sets are disjoint, no final states were reached
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{config::ReadFAConfig, nfa::{_backtrack, acceptor}};
    use super::NFA;

    #[test]
    fn test_nfa01() {
        // Make sure the other approach for NFA evaluation gives the same results
        // as the backtracking approach.
        let mut path = PathBuf::from("./test_configs/");
        // test nfa01.json - strings ending in ab
        path.push("nfa01.json");
        let nfa = NFA::from_file(path);
        let test_cases = ["abbbab", "bab", "b", "", "bbbbbbbbbba"];
        for input in test_cases {
            assert_eq!(_backtrack(&nfa, nfa.initial_state, 0, &input.clone().chars().collect::<Vec<char>>()), acceptor(&nfa, input.to_owned()));
        }
    }

    #[test]
    fn test_nfa02() {
        let mut path = PathBuf::from("./test_configs/");
        // test nfa02.json - strings starting with a
        path.push("nfa02.json");
        let nfa = NFA::from_file(path);
        let test_cases = ["abbbab", "bab", "b", "", "bbbbbbbbbba", "a"];
        for input in test_cases {
            assert_eq!(_backtrack(&nfa, nfa.initial_state, 0, &input.clone().chars().collect::<Vec<char>>()), acceptor(&nfa, input.to_owned()));
        }
    }

    #[test]
    fn test_nfa03() {
        let mut path = PathBuf::from("./test_configs/");
        // test nfa03.json - strings matching a regex b*a
        path.push("nfa03.json");
        let nfa = NFA::from_file(path);
        let test_cases = ["abbbab", "bab", "b", "a", "bba", "bbbba"];
        for input in test_cases {
            assert_eq!(_backtrack(&nfa, nfa.initial_state, 0, &input.clone().chars().collect::<Vec<char>>()), acceptor(&nfa, input.to_owned()));
        }
    }
}