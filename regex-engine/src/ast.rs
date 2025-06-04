pub enum RegexAst {
    Dot,
    Literal(char),
    Concat(Box<RegexAst>, Box<RegexAst>),
    Alternate(Box<RegexAst>, Box<RegexAst>),
    KleeneStar(Box<RegexAst>),
}
