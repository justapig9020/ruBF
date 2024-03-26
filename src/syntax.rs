use crate::token::Token;
use anyhow::{anyhow, Result};

pub struct Source {
    code: Vec<Token>,
    cursor: usize,
}

impl From<&str> for Source {
    fn from(code: &str) -> Self {
        let code = code.chars().map(Token::from).collect();
        Self { code, cursor: 0 }
    }
}

impl Source {
    fn next_symbol(&mut self) -> Token {
        let sym = self.code.get(self.cursor).unwrap_or(&Token::EoF);
        if sym != &Token::EoF {
            self.cursor += 1;
        }
        *sym
    }
    fn snapshot(&self) -> Snapshot {
        self.cursor
    }
    fn restore(&mut self, snapshot: Snapshot) {
        self.cursor = snapshot;
    }
}

type Snapshot = usize;
pub struct Program {
    pub ast: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Loop(Vec<Expression>),
    Operator(Token),
    Skip,
}

impl TryFrom<&str> for Program {
    type Error = anyhow::Error;
    fn try_from(code: &str) -> Result<Self> {
        let source = Source::from(code);
        Program::try_from(source)
    }
}

fn skip(source: &mut Source) -> bool {
    let results = [
        try_parse(source, comment),
        try_parse(source, eol),
        try_parse(source, white_space),
    ];
    results.iter().map(|r| r.is_ok()).any(|b| b)
}

impl TryFrom<Source> for Program {
    type Error = anyhow::Error;
    fn try_from(mut source: Source) -> Result<Self> {
        let mut ast = Vec::new();
        loop {
            match try_parse(&mut source, exp) {
                Ok(exp) => {
                    ast.push(exp);
                }
                Err(e) => {
                    let sym = source.next_symbol();
                    return if sym == Token::EoF {
                        Ok(Self { ast })
                    } else {
                        Err(e)
                    };
                }
            }
        }
    }
}

fn exp(source: &mut Source) -> Result<Expression> {
    if skip(source) {
        return Ok(Expression::Skip);
    }
    let lp_result = try_parse(source, lp);
    let lp_err = if let Err(err) = lp_result {
        err
    } else {
        return lp_result;
    };

    let sym_result = try_parse(source, sym);
    let sym_err = if let Err(err) = sym_result {
        err
    } else {
        return sym_result;
    };

    Err(anyhow!("exp: {lp_err:?} {sym_err:?}"))
}

fn exp_list(source: &mut Source) -> Result<Vec<Expression>> {
    let mut result = Vec::new();
    while let Ok(exp) = try_parse(source, exp) {
        result.push(exp);
    }
    if result.is_empty() {
        Err(anyhow!("Expect at least one expression"))
    } else {
        Ok(result)
    }
}

fn lp(source: &mut Source) -> Result<Expression> {
    let symbol = source.next_symbol();
    if Token::LeftBracket != symbol {
        return Err(anyhow!("Expect left bracket, but got {:?}", symbol));
    }

    let exp_list = try_parse(source, exp_list)?;

    let symbol = source.next_symbol();
    if Token::RightBracket != symbol {
        return Err(anyhow!("Expect right bracket, but got {:?}", symbol));
    }
    Ok(Expression::Loop(exp_list))
}

fn sym(source: &mut Source) -> Result<Expression> {
    let symbol = source.next_symbol();
    match symbol {
        Token::RightBracket | Token::LeftBracket | Token::EoF => {
            Err(anyhow!("Expect symbols, but got {:?}", symbol))
        }
        _ => Ok(Expression::Operator(symbol)),
    }
}

fn comment(source: &mut Source) -> Result<()> {
    for _ in 0..2 {
        if source.next_symbol() != Token::Slash {
            return Err(anyhow!("Expect slash"));
        }
    }
    while source.next_symbol() != Token::NewLine {}
    Ok(())
}

fn eol(source: &mut Source) -> Result<()> {
    if source.next_symbol() != Token::NewLine {
        return Err(anyhow!("Expect EoL"));
    }
    Ok(())
}

fn white_space(source: &mut Source) -> Result<()> {
    if source.next_symbol() != Token::WhiteSpace {
        return Err(anyhow!("Expect white space"));
    }
    Ok(())
}

fn try_parse<T>(soruce: &mut Source, rule: fn(&mut Source) -> Result<T>) -> Result<T> {
    let snapshot = soruce.snapshot();
    match rule(soruce) {
        Err(e) => {
            soruce.restore(snapshot);
            Err(e.context("parse error"))
        }
        Ok(exp) => Ok(exp),
    }
}

#[cfg(test)]
mod syntax {
    use super::*;
    use crate::token::Token::*;
    use Expression::*;

    #[test]
    fn test_parser() {
        let code = Source::from("[+]-");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![
                Loop(vec![Expression::Operator(PlusOne)]),
                Operator(MinusOne),
            ]
        );
    }
    #[test]
    fn test_parser_nested_loop() {
        let code = Source::from("[[+]-]");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![Loop(vec![
                Loop(vec![Operator(PlusOne)]),
                Operator(MinusOne)
            ])]
        );
    }
    #[test]
    fn test_comment() {
        let code = Source::from("// comment\n+");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(program.ast, vec![Skip, Operator(PlusOne)]);
    }
    #[test]
    fn test_eol() {
        let code = Source::from("\n+\n-\n");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![Skip, Operator(PlusOne), Skip, Operator(MinusOne), Skip]
        );
    }
    #[test]
    fn test_white_space() {
        let code = Source::from(" +\t-");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![Skip, Operator(PlusOne), Skip, Operator(MinusOne)]
        );
    }
}
