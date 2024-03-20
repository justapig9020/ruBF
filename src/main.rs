mod brainfuck;
mod syntax;
mod tap;
mod token;
use anyhow::Result;
use clap::Parser;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program: String,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut input: Box<dyn Read> = if let Some(input_name) = args.input {
        let input_file = File::open(input_name)?;
        Box::new(input_file)
    } else {
        let stdin = io::stdin();
        Box::new(stdin)
    };
    let mut output: Box<dyn Write> = if let Some(output_name) = args.output {
        let output_file = File::create(output_name)?;
        Box::new(output_file)
    } else {
        let stdout = io::stdout();
        Box::new(stdout)
    };
    let program = read_to_string(args.program)?;
    let mut vm = brainfuck::VirtualMachine::new(&mut input, &mut output);
    vm.run(&program)?;
    Ok(())
}
