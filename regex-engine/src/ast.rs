#[derive(Debug, Clone, PartialEq)]
pub enum RegexAst {
    Dot,
    Literal(char),
    Concat(Box<RegexAst>, Box<RegexAst>),
    Alternate(Box<RegexAst>, Box<RegexAst>),
    KleeneStar(Box<RegexAst>),
    Plus(Box<RegexAst>),
    Qmark(Box<RegexAst>),
}
