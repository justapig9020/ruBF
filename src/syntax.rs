use crate::symbol::Symbol;
use anyhow::{anyhow, Result};

pub struct Program {
    code: Vec<Symbol>,
    cursor: usize,
}

impl From<&str> for Program {
    fn from(code: &str) -> Self {
        let code = code.chars().map(Symbol::from).collect();
        Self { code, cursor: 0 }
    }
}

impl Program {
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
type AST = Vec<Expression>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expression {
    Loop(Vec<Expression>),
    Operator(Symbol),
}

impl TryFrom<Program> for AST {
    type Error = anyhow::Error;
    fn try_from(mut program: Program) -> Result<Self> {
        let mut result = Vec::new();
        loop {
            match parse(&mut program, exp) {
                Ok(exp) => {
                    result.push(exp);
                }
                Err(e) => {
                    return if program.next_symbol() == Symbol::EoF {
                        Ok(result)
                    } else {
                        Err(e)
                    };
                }
            }
        }
    }
}

fn exp(program: &mut Program) -> Result<Expression> {
    let lp_result = parse(program, lp);
    let lp_err = if let Err(err) = lp_result {
        err
    } else {
        return lp_result;
    };

    let sym_result = parse(program, sym);
    let sym_err = if let Err(err) = sym_result {
        err
    } else {
        return sym_result;
    };

    Err(anyhow!("parse error: ").context(lp_err).context(sym_err))
}

fn exp_list(program: &mut Program) -> Result<Vec<Expression>> {
    let mut result = Vec::new();
    while let Ok(exp) = parse(program, exp) {
        result.push(exp);
    }
    if result.is_empty() {
        Err(anyhow!("Expect at least one expression"))
    } else {
        Ok(result)
    }
}

fn lp(program: &mut Program) -> Result<Expression> {
    let symbol = program.next_symbol();
    if Symbol::LeftBracket != symbol {
        return Err(anyhow!("Expect left bracket, but got {:?}", symbol));
    }

    let exp_list = parse(program, exp_list)?;

    let symbol = program.next_symbol();
    if Symbol::RightBracket != symbol {
        return Err(anyhow!("Expect right bracket, but got {:?}", symbol));
    }
    Ok(Expression::Loop(exp_list))
}

fn sym(program: &mut Program) -> Result<Expression> {
    let symbol = program.next_symbol();
    match symbol {
        Symbol::RightBracket | Symbol::LeftBracket | Symbol::EoF => {
            Err(anyhow!("Expect symbols, but got {:?}", symbol))
        }
        _ => Ok(Expression::Operator(symbol)),
    }
}

fn parse<T>(program: &mut Program, rule: fn(&mut Program) -> Result<T>) -> Result<T> {
    let snapshot = program.snapshot();
    if let Ok(exp) = rule(program) {
        return Ok(exp);
    }
    program.restore(snapshot);
    Err(anyhow!("parse error"))
}

#[cfg(test)]
mod syntax {
    use super::*;
    use crate::symbol::Symbol::*;
    #[test]
    fn test_parser() {
        let code = Program::from("[+]-");
        let program = AST::try_from(code);
        assert_eq!(
            program.unwrap_or_else(|e| panic!("Error: {}", e)),
            vec![
                Expression::Loop(vec![Expression::Operator(PlusOne)]),
                Expression::Operator(MinusOne),
            ]
        );
    }
}
