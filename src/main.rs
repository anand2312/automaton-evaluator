mod dfa;
mod fa;
mod nfa;

use clap::{builder::PossibleValue, Parser, ValueEnum};
use dfa::DFA;
use fa::{Acceptor, ReadFAConfig};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Arguments {
    /// The configuration file describing the automaton to be run
    file: PathBuf,
    #[clap(short, long, value_enum)]
    /// The type of automaton to run
    automaton: AutomatonType,
    #[clap(short, long)]
    /// The string to test
    string: String,
}

#[derive(Copy, Clone, Debug)]
enum AutomatonType {
    DFA,
}

impl ValueEnum for AutomatonType {
    fn value_variants<'a>() -> &'a [Self] {
        &[AutomatonType::DFA]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            AutomatonType::DFA => {
                PossibleValue::new("dfa").help("Deterministic Finite State Automata")
            }
        })
    }
}

fn main() {
    let args = Arguments::parse();
    match args.automaton {
        AutomatonType::DFA => {
            let machine = DFA::from_file(args.file);
            let result = if machine.test_string(args.string) {
                "Accepted"
            } else {
                "Rejected"
            };
            println!("{}", result);
        }
    }
}
