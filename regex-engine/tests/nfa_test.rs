use regex_engine::asttonfa::from_ast;
use regex_engine::nfa::Transition;
use regex_engine::parser::{ParseError, parse};

#[test]
fn builds_concat_nfa() {
    let ast = parse("ab").unwrap();
    let nfa = from_ast(&ast);
    assert_eq!(nfa.states.len(), 4); // 'a', ε, 'b'

    match &nfa.states[0].transitions[0] {
        Transition::Char('a', _) => {}
        _ => panic!("Expected transition on 'a'"),
    }

    match &nfa.states[1].transitions[0] {
        Transition::Epsilon(_) => {}
        _ => panic!("Expected epsilon transition between a and b"),
    }

    match &nfa.states[2].transitions[0] {
        Transition::Char('b', _) => {}
        _ => panic!("Expected transition on 'b'"),
    }
}

#[test]
fn builds_kleene_star_nfa() {
    let ast = parse("a*").unwrap();
    let nfa = from_ast(&ast);

    let epsilon_count = nfa
        .states
        .iter()
        .flat_map(|s| &s.transitions)
        .filter(|t| matches!(t, Transition::Epsilon(_)))
        .count();

    assert!(
        epsilon_count >= 3,
        "Expected at least 3 ε-transitions for star"
    );
}

#[test]
fn builds_dot_nfa() {
    let ast = parse(".").unwrap();
    let nfa = from_ast(&ast);

    match &nfa.states[0].transitions[0] {
        Transition::Any(_) => {}
        _ => panic!("Expected wildcard transition for '.'"),
    }
}

#[test]
fn builds_alternate_nfa() {
    let ast = parse("a|b").unwrap();
    let nfa = from_ast(&ast);

    // Alternate should have at least 6 states: split, a, aε, b, bε, merge
    assert!(nfa.states.len() >= 6);

    let epsilon_count = nfa
        .states
        .iter()
        .flat_map(|s| &s.transitions)
        .filter(|t| matches!(t, Transition::Epsilon(_)))
        .count();

    assert!(
        epsilon_count >= 4,
        "Expected at least 4 ε-transitions for alternation"
    );
}

#[test]
fn handles_unexpected_char_error() {
    let err = parse("*a").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedChar('*')));
}

#[test]
fn handles_unmatched_paren_error() {
    let err = parse("(ab").unwrap_err();
    assert!(matches!(err, ParseError::UnmatchedParen));
}

#[test]
fn handles_empty_alternation_error() {
    let err = parse("a|").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedEnd));
}
