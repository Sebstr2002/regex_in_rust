use regex_engine::ast::RegexAst;
use regex_engine::parser::{ParseError, parse};

#[test]
fn parses_literal_sequence() {
    let ast = parse("abc").unwrap();
    assert_eq!(
        ast,
        RegexAst::Concat(
            Box::new(RegexAst::Concat(
                Box::new(RegexAst::Literal('a')),
                Box::new(RegexAst::Literal('b')),
            )),
            Box::new(RegexAst::Literal('c')),
        )
    );
}

#[test]
fn parses_alternation() {
    let ast = parse("a|b").unwrap();
    assert_eq!(
        ast,
        RegexAst::Alternate(
            Box::new(RegexAst::Literal('a')),
            Box::new(RegexAst::Literal('b')),
        )
    );
}

#[test]
fn parses_group_with_kleene_star() {
    let ast = parse("(ab)*").unwrap();
    assert_eq!(
        ast,
        RegexAst::KleeneStar(Box::new(RegexAst::Concat(
            Box::new(RegexAst::Literal('a')),
            Box::new(RegexAst::Literal('b')),
        )))
    );
}

#[test]
fn parses_dot_and_star() {
    let ast = parse(".*").unwrap();
    assert_eq!(ast, RegexAst::KleeneStar(Box::new(RegexAst::Dot)));
}

#[test]
fn parses_group_with_qmark() {
    let ast = parse("(ab)?").unwrap();
    assert_eq!(
        ast,
        RegexAst::Qmark(Box::new(RegexAst::Concat(
            Box::new(RegexAst::Literal('a')),
            Box::new(RegexAst::Literal('b')),
        )))
    );
}

#[test]
fn parses_dot_and_qmark() {
    let ast = parse(".?").unwrap();
    assert_eq!(ast, RegexAst::Qmark(Box::new(RegexAst::Dot)));
}

#[test]
fn parses_group_with_plus() {
    let ast = parse("(ab)+").unwrap();
    assert_eq!(
        ast,
        RegexAst::Plus(Box::new(RegexAst::Concat(
            Box::new(RegexAst::Literal('a')),
            Box::new(RegexAst::Literal('b')),
        )))
    );
}

#[test]
fn parses_dot_and_plus() {
    let ast = parse(".+").unwrap();
    assert_eq!(ast, RegexAst::Plus(Box::new(RegexAst::Dot)));
}
#[test]
fn handles_unexpected_char() {
    let err = parse("*").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedChar('*')));
}

#[test]
fn handles_unmatched_paren() {
    let err = parse("(ab").unwrap_err();
    assert!(matches!(err, ParseError::UnmatchedParen));
}

#[test]
fn handles_empty_alternation() {
    let err = parse("a|").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedEnd));
}

