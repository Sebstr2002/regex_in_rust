use crate::ast::RegexAst;
use crate::nfa;

pub fn from_ast(ast: &RegexAst) -> nfa::NFA {
    let mut builder = NfaBuilder::new();
    let frag = builder.build(ast);
    nfa::NFA {
        states: builder.states,
        start: frag.start,
        accept: frag.accept,
    }
}

// this struct will help us analyze the tree and convert notes into fragments
// and keep track of transitions
struct NfaBuilder {
    states: Vec<nfa::State>,
}

impl NfaBuilder {
    // constructor
    fn new() -> Self {
        Self { states: Vec::new() }
    }
    // adds an empty state and returns index of the added state
    fn add_state(&mut self) -> usize {
        let idx = self.states.len();
        self.states.push(nfa::State {
            transitions: vec![],
        });
        idx
    }
    //build will collect all states from ast and
    fn build(&mut self, ast: &RegexAst) -> nfa::NfaFragment {
        match ast {
            RegexAst::Literal(c) => self.build_literal(*c),
            RegexAst::Dot => self.build_dot(),
            RegexAst::Concat(a, b) => {
                let left = self.build(a);
                let right = self.build(b);
                self.link(left.accept, nfa::Transition::Epsilon(right.start));
                nfa::NfaFragment {
                    start: left.start,
                    accept: right.accept,
                }
            }
            RegexAst::Alternate(a, b) => {
                let s = self.add_state();
                let left = self.build(a);
                let right = self.build(b);
                let accept = self.add_state();

                self.link(s, nfa::Transition::Epsilon(left.start));
                self.link(s, nfa::Transition::Epsilon(right.start));
                self.link(left.accept, nfa::Transition::Epsilon(accept));
                self.link(right.accept, nfa::Transition::Epsilon(accept));

                nfa::NfaFragment { start: s, accept }
            }
            RegexAst::KleeneStar(a) => {
                let s = self.add_state();
                let frag = self.build(a);
                let accept = self.add_state();

                self.link(s, nfa::Transition::Epsilon(frag.start));
                self.link(s, nfa::Transition::Epsilon(accept));
                self.link(frag.accept, nfa::Transition::Epsilon(frag.start));
                self.link(frag.accept, nfa::Transition::Epsilon(accept));

                nfa::NfaFragment { start: s, accept }
            }
        }
    }

    fn build_literal(&mut self, c: char) -> nfa::NfaFragment {
        let start = self.add_state();
        let accept = self.add_state();
        self.link(start, nfa::Transition::Char(c, accept));
        nfa::NfaFragment { start, accept }
    }
    fn build_dot(&mut self) -> nfa::NfaFragment {
        let start = self.add_state();
        let accept = self.add_state();
        self.link(start, nfa::Transition::Any(accept));
        nfa::NfaFragment { start, accept }
    }

    fn link(&mut self, from: usize, trans: nfa::Transition) {
        self.states[from].transitions.push(trans);
    }
}
