use crate::symbol::Symbol;
use anyhow::{anyhow, Result};

pub struct Source {
    code: Vec<Symbol>,
    cursor: usize,
}

impl From<&str> for Source {
    fn from(code: &str) -> Self {
        let code = code.chars().map(Symbol::from).collect();
        Self { code, cursor: 0 }
    }
}

impl Source {
    fn next_symbol(&mut self) -> Symbol {
        let sym = self.code.get(self.cursor).or(Some(&Symbol::EoF)).unwrap();
        self.cursor += 1;
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
    Operator(Symbol),
}

impl TryFrom<&str> for Program {
    type Error = anyhow::Error;
    fn try_from(code: &str) -> Result<Self> {
        let source = Source::from(code);
        Program::try_from(source)
    }
}

impl TryFrom<Source> for Program {
    type Error = anyhow::Error;
    fn try_from(mut source: Source) -> Result<Self> {
        let mut ast = Vec::new();
        loop {
            match parse(&mut source, exp) {
                Ok(exp) => {
                    ast.push(exp);
                }
                Err(e) => {
                    return if source.next_symbol() == Symbol::EoF {
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
    let lp_result = parse(source, lp);
    let lp_err = if let Err(err) = lp_result {
        err
    } else {
        return lp_result;
    };

    let sym_result = parse(source, sym);
    let sym_err = if let Err(err) = sym_result {
        err
    } else {
        return sym_result;
    };

    Err(anyhow!("parse error: ").context(lp_err).context(sym_err))
}

fn exp_list(source: &mut Source) -> Result<Vec<Expression>> {
    let mut result = Vec::new();
    while let Ok(exp) = parse(source, exp) {
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
    if Symbol::LeftBracket != symbol {
        return Err(anyhow!("Expect left bracket, but got {:?}", symbol));
    }

    let exp_list = parse(source, exp_list)?;

    let symbol = source.next_symbol();
    if Symbol::RightBracket != symbol {
        return Err(anyhow!("Expect right bracket, but got {:?}", symbol));
    }
    Ok(Expression::Loop(exp_list))
}

fn sym(source: &mut Source) -> Result<Expression> {
    let symbol = source.next_symbol();
    match symbol {
        Symbol::RightBracket | Symbol::LeftBracket | Symbol::EoF => {
            Err(anyhow!("Expect symbols, but got {:?}", symbol))
        }
        _ => Ok(Expression::Operator(symbol)),
    }
}

fn parse<T>(soruce: &mut Source, rule: fn(&mut Source) -> Result<T>) -> Result<T> {
    let snapshot = soruce.snapshot();
    if let Ok(exp) = rule(soruce) {
        return Ok(exp);
    }
    soruce.restore(snapshot);
    Err(anyhow!("parse error"))
}

#[cfg(test)]
mod syntax {
    use super::*;
    use crate::symbol::Symbol::*;
    #[test]
    fn test_parser() {
        let code = Source::from("[+]-");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![
                Expression::Loop(vec![Expression::Operator(PlusOne)]),
                Expression::Operator(MinusOne),
            ]
        );
    }
    #[test]
    fn test_parser_nested_loop() {
        let code = Source::from("[[+]-]");
        let program = Program::try_from(code).unwrap_or_else(|e| panic!("Error: {}", e));
        assert_eq!(
            program.ast,
            vec![Expression::Loop(vec![
                Expression::Loop(vec![Expression::Operator(PlusOne)]),
                Expression::Operator(MinusOne)
            ])]
        );
    }
}
