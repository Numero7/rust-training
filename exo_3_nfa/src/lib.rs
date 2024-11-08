use std::collections::HashSet;

pub struct Transition {
    from: State,
    letter: Letter,
    to: State,
}

pub struct Nfa {
    states: HashSet<State>,
    initial: HashSet<State>,
    accepting: HashSet<State>,
    transitions: Vec<Transition>,
}

type State = usize;
type Letter = char;

impl Nfa {
    pub fn new(n_states: usize) -> Self {
        return Nfa {
            states: (1..=n_states).collect(),
            initial: HashSet::new(),
            accepting: HashSet::new(),
            transitions: vec![],
        };
    }

    pub fn add_transition(&mut self, from: State, to: State, label: char) {
        self.transitions.push(Transition {
            from: from,
            letter: label,
            to: to,
        });
    }

    pub fn add_initial(&mut self, q: State) {
        self.states.insert(q);
        self.initial.insert(q);
    }

    pub fn add_final(&mut self, q: State) {
        self.states.insert(q);
        self.accepting.insert(q);
    }

    fn step(&self, states: HashSet<State>, a: char) -> HashSet<State> {
        return states
            .into_iter()
            .flat_map(|q| {
                self.transitions
                    .iter()
                    .filter(move |t| t.from == q && t.letter == a)
                    .map(|t| t.to.clone())
            })
            .collect();
    }

    pub fn accepts(&self, s: &str) -> bool {
        let mut reachable_states = self.initial.clone();
        for a in s.chars() {
            reachable_states = self.step(reachable_states, a)
        }
        return reachable_states.iter().any(|q| self.accepting.contains(q));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parity() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'a');
        nfa.add_transition(0, 0, 'b');
        nfa.add_transition(1, 1, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababbaba"));
        assert!(nfa.accepts("aabbaa"));
        assert!(!nfa.accepts("abbaa"));
        assert!(!nfa.accepts("aaa"));
    }

    #[test]
    fn a_b_star() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababab"));
        assert!(nfa.accepts("abab"));
        assert!(!nfa.accepts("aba"));
        assert!(!nfa.accepts("aababa"));
        assert!(!nfa.accepts("abababba"));
    }
}
