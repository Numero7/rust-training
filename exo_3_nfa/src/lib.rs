use std::collections::HashSet;

pub struct Nfa {
    // TODO: add members
}

type State = usize;

impl Nfa {
    pub fn new(n_states: usize) -> Self {
        todo!("Create a new Nfa with n_states states")
    }

    pub fn add_transition(&mut self, from: State, to: State, label: char) {
        todo!("Add a transition to Self")
    }

    pub fn add_initial(&mut self, q: State) {
        todo!("Add q to the set of initial states");
    }

    pub fn add_final(&mut self, q: State) {
        todo!("Add q to the set of final states");
    }

    fn step(&self, states: HashSet<State>, a: char) -> HashSet<State> {
        todo!("Compute the transition labeled by a in Self")
    }

    pub fn accepts(&self, s: &str) -> bool {
        todo!("Test whether s is in the language of Self")
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
