use crate::syntax::{Expression, Program};
use crate::tape::{Direction, Tape};
use crate::token::Token;
use anyhow::Result;
use std::io::{Read, Write};

pub struct VirtualMachine<'a> {
    tape: Tape,
    debug: bool,
    input: &'a mut dyn Read,
    output: &'a mut dyn Write,
}

impl std::fmt::Debug for VirtualMachine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tape)
    }
}

impl<'a> VirtualMachine<'a> {
    pub fn new(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::default(),
            debug: false,
            input,
            output,
        }
    }
    pub fn new_debug(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::default(),
            debug: true,
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
                Skip => {}
            }
        }
        Ok(())
    }
    fn execute_loop(&mut self, ast: &[Expression]) -> Result<()> {
        while self.tape.get() != 0 {
            self.execute_ast(ast)?;
        }
        Ok(())
    }
    fn execute_symbol(&mut self, token: Token) -> Result<()> {
        use Token::*;
        if self.debug {
            writeln!(self.output, "tape: {:?}", self.tape)?;
            writeln!(self.output, "symbol: {:?}", token)?;
        }
        match token {
            PlusOne => self.tape.set(self.tape.get().wrapping_add(1)),
            MinusOne => self.tape.set(self.tape.get().wrapping_sub(1)),
            RightShift => self.tape.move_cursor(Direction::Right),
            LeftShift => self.tape.move_cursor(Direction::Left),
            Input => {
                let mut buf = [0; 1];
                loop {
                    self.input.read_exact(&mut buf)?;
                    if buf[0] != b'\n' {
                        break;
                    }
                }
                self.tape.set(buf[0]);
            }
            Output => {
                let value = self.tape.get();
                write!(self.output, "{}", char::from(value))?;
                self.output.flush()?;
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
