# automaton-evaluator
I was bored the night before my Formal Languages and Automata Theory lab examination, so I started working on this instead... \
I got the idea while playing around with [JFLAP](https://www.jflap.org/)

## Roadmap
List of features I'd like to implement (highly unlikely):
- [x] Deterministic Finite State Automata
- [] Non-deterministic Finite State Automata
- [] Pushdown Automata
- [] Simple Turing Machines
- [] Visualize automata described by a config file
- [] GUI???

## Example usage
```bash
$ cargo run -- /path/to/config.json --automaton=dfa --string=abaaaba
Accepted
```

## Example Config
Example configuration files for any automatons that have been implemented so far are in the `test_configs` directory.
