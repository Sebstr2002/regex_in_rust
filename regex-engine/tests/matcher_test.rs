use regex_engine::{asttonfa::from_ast, matcher::Match, parser::parse};

#[test]
fn test_greedy_find() {
    let ast = parse("a*").unwrap();
    let nfa = from_ast(&ast);
    let input = "aaa";

    assert_eq!(nfa.find(input), Some(Match { start: 0, end: 3 }));
}

#[test]
fn qmark_find() {
    let ast = parse("(ab)?").unwrap();
    let nfa = from_ast(&ast);
    let input = "ab";

    assert_eq!(
        nfa.find_all(input),
        vec![Match { start: 0, end: 2 }, Match { start: 2, end: 2 },]
    );
}

#[test]
fn plus_find() {
    let ast = parse("(ab)+").unwrap();
    let nfa = from_ast(&ast);
    let input = "ab abab";

    assert_eq!(
        nfa.find_all(input),
        vec![Match { start: 0, end: 2 }, Match { start: 3, end: 7 },]
    );
}

#[test]
fn find_escaped() {
    let ast = parse("a/*").unwrap();
    let nfa = from_ast(&ast);
    let input = "a*";

    assert_eq!(nfa.find_all(input), vec![Match { start: 0, end: 2 }]);
}

#[test]
fn test_find_all() {
    let ast = parse("ab").unwrap();
    let nfa = from_ast(&ast);
    let input = "ab ab ab";

    assert_eq!(
        nfa.find_all(input),
        vec![
            Match { start: 0, end: 2 },
            Match { start: 3, end: 5 },
            Match { start: 6, end: 8 },
        ]
    );
}
