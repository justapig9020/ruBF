use crate::symbol::Symbol;
use crate::syntax::{Expression, Program};
use crate::tap::{Direction, Tap};
use anyhow::Result;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 100;

pub struct VirtualMachine<'a> {
    tap: Tap,
    input: &'a mut dyn Read,
    output: &'a mut dyn Write,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
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

    fn run_and_assert(program: &str, expect: &str) {
        let mut input = Cursor::new(Vec::with_capacity(expect.len()));
        let mut output = Cursor::new(Vec::with_capacity(expect.len()));

        let mut bf = VirtualMachine::new(&mut input, &mut output);
        bf.run(program).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(output, expect);
    }

    #[test]
    fn hello_world() {
        let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let expect = "Hello World!\n";
        run_and_assert(program, expect);
    }

    #[test]
    fn adder() {
        let program = "++++++++++++++++++++++++++++++>+++[-<+>]<.";
        let expect = "!";
        run_and_assert(program, expect)
    }
}
