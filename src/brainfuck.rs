use crate::symbol::Symbol;
use crate::syntax::{Expression, Program, Source};
use crate::tap::{Direction, Tap};
use anyhow::{anyhow, Result};
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 100;

pub struct VirtualMachine<'a, R, W> {
    tap: Tap,
    input: &'a mut R,
    output: &'a mut W,
}

impl<'a, R, W> VirtualMachine<'a, R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(input: &'a mut R, output: &'a mut W) -> Self {
        Self {
            tap: Tap::default(),
            input,
            output,
        }
    }
    pub fn run(&mut self, source_code: &str) -> Result<()> {
        let program = Program::try_from(source_code)?;
        self.execute_ast(&program.ast)?;
        Ok(())
    }
    fn execute_ast(&mut self, ast: &[Expression]) -> Result<()> {
        use Expression::*;
        for exp in ast.iter() {
            match exp {
                Loop(ast) => self.execute_loop(ast)?,
                Operator(sym) => self.execute_symbol(*sym)?,
            }
        }
        Ok(())
    }
    fn execute_loop(&mut self, ast: &[Expression]) -> Result<()> {
        while self.tap.get() != 0 {
            self.execute_ast(ast)?;
        }
        Ok(())
    }
    fn execute_symbol(&mut self, symbol: Symbol) -> Result<()> {
        use Symbol::*;
        match symbol {
            PlusOne => self.tap.set(self.tap.get().wrapping_add(1)),
            MinusOne => self.tap.set(self.tap.get().wrapping_sub(1)),
            RightShift => self.tap.move_cursor(Direction::Right),
            LeftShift => self.tap.move_cursor(Direction::Left),
            Input => {
                let mut buf = [0; 1];
                self.input.read_exact(&mut buf)?;
                self.tap.set(buf[0]);
            }
            Output => {
                let value = self.tap.get();
                write!(self.output, "{}", char::from(value))?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod brainfuck {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn hello_world() {
        let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let expect_output = "Hello World!\n";
        let mut input = Cursor::new(Vec::with_capacity(expect_output.len()));
        let mut output = Cursor::new(Vec::with_capacity(expect_output.len()));

        let mut bf = VirtualMachine::new(&mut input, &mut output);
        bf.run(program).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(output, expect_output);
    }
}
