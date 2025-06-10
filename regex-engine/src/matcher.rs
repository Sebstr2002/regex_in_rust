use crate::nfa::{NFA, Transition};
use std::{collections::HashSet, thread::current};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl NFA {
    pub fn find(&self, input: &str) -> Option<Match> {
        let chars: Vec<char> = input.chars().collect();

        for start_idx in 0..=chars.len() {
            let mut current = self.epsilon_closure(&[self.start]);
            let mut matched: Option<usize> = None;

            for i in start_idx..chars.len() {
                current = self.step(&current, chars[i]);

                if current.is_empty() {
                    break;
                }

                if current.contains(&self.accept) {
                    matched = Some(i + 1); // greedy matching
                }
            }

            if matched.is_none() && current.contains(&self.accept) {
                matched = Some(start_idx);
            }

            if let Some(end) = matched {
                return Some(Match {
                    start: start_idx,
                    end,
                });
            }
        }

        None
    }

    pub fn find_all(&self, input: &str) -> Vec<Match> {
        let chars: Vec<char> = input.chars().collect();
        let mut results = Vec::new();
        let mut index = 0;

        while index <= chars.len() {
            let mut current = self.epsilon_closure(&[self.start]);
            let mut matched: Option<usize> = None;

            for i in index..chars.len() {
                current = self.step(&current, chars[i]);

                if current.is_empty() {
                    break;
                }

                if current.contains(&self.accept) {
                    matched = Some(i + 1);
                }
            }

            if matched.is_none() && current.contains(&self.accept) {
                matched = Some(index);
            }

            if let Some(end) = matched {
                results.push(Match { start: index, end });
                if end == index {
                    index += 1;
                } else {
                    index = end;
                }
            } else {
                index += 1;
            }
        }

        results
    }
    fn epsilon_closure(&self, states: &[usize]) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut stack = states.to_vec();

        while let Some(state) = stack.pop() {
            if visited.insert(state) {
                for t in &self.states[state].transitions {
                    if let Transition::Epsilon(target) = t {
                        stack.push(*target);
                    }
                }
            }
        }

        visited
    }

    fn step(&self, current: &HashSet<usize>, input: char) -> HashSet<usize> {
        // current is a
        // hashset because we want an epsilon closure of current state not just the state
        let mut next = HashSet::new();

        for &state in current {
            for t in &self.states[state].transitions {
                match t {
                    Transition::Any(target) => {
                        next.extend(self.epsilon_closure(&[*target]));
                    }
                    Transition::Char(expected, target) if *expected == input => {
                        next.extend(self.epsilon_closure(&[*target]));
                    }
                    _ => {}
                }
            }
        }

        next
    }
}
