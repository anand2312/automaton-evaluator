use automaton_evaluator::{dfa::DFA, nfa::NFA, config::ReadFAConfig, traits::Acceptor};
use clap::{builder::PossibleValue, Parser, ValueEnum};
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
    NFA,
}

impl ValueEnum for AutomatonType {
    fn value_variants<'a>() -> &'a [Self] {
        &[AutomatonType::DFA, AutomatonType::NFA]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            AutomatonType::DFA => {
                PossibleValue::new("dfa").help("Deterministic Finite State Automata")
            }
            AutomatonType::NFA => {
                PossibleValue::new("nfa").help("Non-deterministic Finite State Automata")
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
        AutomatonType::NFA => {
            let machine = NFA::from_file(args.file);
            let result = if machine.test_string(args.string) {
                "Accepted"
            } else {
                "Rejected"
            };
            println!("{}", result);
        }
    }
}
