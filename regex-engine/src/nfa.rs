#[derive(Debug, Clone)]
pub struct State {
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone)]
pub enum Transition {
    Char(char, usize),
    Any(usize),
    Epsilon(usize),
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: Vec<State>,
    pub start: usize,
    pub accept: usize,
}

pub struct NfaFragment {
    pub start: usize,
    pub accept: usize,
}
