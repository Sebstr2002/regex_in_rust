// import the syntax tree
use crate::ast::RegexAst;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedChar(char),
    UnmatchedParen,
}

// parser function which takes a regex and forms a syntax tree
pub fn parse(input: &str) -> Result<RegexAst, ParseError> {
    let mut parser = Parser::new(input);
    return parser.parse_exp();
}

// parser structure
struct Parser<'a> {
    chars: std::str::Chars<'a>, // available as long as regex is available
    current: Option<char>,
}

// implementations
impl<'a> Parser<'a> {
    // constructor
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        Self { chars, current }
    }

    // move current to next char
    fn bump(&mut self) {
        self.current = self.chars.next();
    }

    fn parse_exp(&mut self) -> Result<RegexAst, ParseError> {
        self.parse_alternation()
    }
    // now implement each regex pattern
    // starst at lowest order alternation
    fn parse_alternation(&mut self) -> Result<RegexAst, ParseError> {
        // push regex onto lower level analysis
        let mut left = self.parse_concat()?;

        while self.current == Some('|') {
            self.bump(); // kills |
            // push regex onto lower level analysis
            let right = self.parse_concat()?;
            left = RegexAst::Alternate(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    // next level is the concat level
    fn parse_concat(&mut self) -> Result<RegexAst, ParseError> {
        let mut parts = Vec::new();

        while let Some(c) = self.current {
            // ) and | marks where each part ends
            if c == ')' || c == '|' {
                break;
            }
            // push regex onto lower level analysis
            let node = self.parse_kleene()?;
            parts.push(node);
        }

        //now make the parts vector into nodes
        let mut iter = parts.into_iter();
        let mut result = iter.next().ok_or(ParseError::UnexpectedEnd)?;

        for part in iter {
            result = RegexAst::Concat(Box::new(result), Box::new(part));
        }
        Ok(result)
    }

    // next level is the star level
    fn parse_kleene(&mut self) -> Result<RegexAst, ParseError> {
        let mut atom = self.parse_atom()?;

        while self.current == Some('*') {
            self.bump();
            atom = RegexAst::KleeneStar(Box::new(atom));
        }
        while self.current == Some('?') {
            self.bump();
            atom = RegexAst::Qmark(Box::new(atom));
        }
        while self.current == Some('+') {
            self.bump();
            atom = RegexAst::Plus(Box::new(atom));
        }
        Ok(atom)
    }
    // ends at highest order atom so a literal or a .
    fn parse_atom(&mut self) -> Result<RegexAst, ParseError> {
        match self.current {
            Some('/') => {
                self.bump();
                if let Some(escaped) = self.current {
                    self.bump();
                    return Ok(RegexAst::Literal(escaped));
                } else {
                    return Err(ParseError::UnexpectedEnd);
                }
            }
            Some('(') => {
                // signals a start of subexpression
                self.bump();
                let expr = self.parse_exp()?;
                if self.current != Some(')') {
                    return Err(ParseError::UnmatchedParen);
                }
                self.bump();
                Ok(expr)
            }
            Some('.') => {
                self.bump();
                Ok(RegexAst::Dot)
            }
            Some(c) if !is_metachar(c) => {
                self.bump();
                Ok(RegexAst::Literal(c))
            }
            Some(c) => Err(ParseError::UnexpectedChar(c)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }
}

fn is_metachar(c: char) -> bool {
    matches!(c, '*' | '(' | ')' | '|' | '+' | '?')
}
