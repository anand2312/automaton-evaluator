# automaton-evaluator
I was bored the night before my Formal Languages and Automata Theory lab examination, so I started working on this instead... \
I got the idea while playing around with [JFLAP](https://www.jflap.org/).

## Roadmap
List of features I'd like to implement (highly unlikely):
- [x] Deterministic Finite State Automata
- [ ] Non-deterministic Finite State Automata
- [ ] Pushdown Automata
- [ ] Simple Turing Machines
- [ ] Visualize automata described by a config file
- [ ] GUI???

## Example usage
```bash
$ cargo run -- /path/to/config.json --automaton=dfa --string=abaaaba
Accepted
```

## Example Config
Example configuration files for any automatons that have been implemented so far are in the `test_configs` directory.
### DFA
- `dfa01.json`: Accepts all strings over $\Sigma = \{a, b\}$ which have `aa` as a substring
- `dfa02.json`: Accepts all strings over $\Sigma = \{0, 1\}$ which have odd number of 1's and odd number of 0's.
