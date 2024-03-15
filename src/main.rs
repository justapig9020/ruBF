mod brainfuck;
mod symbol;
mod syntax;
mod tap;
use anyhow::Result;
use clap::Parser;
use std::fs::read_to_string;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let program = read_to_string(args.program)?;
    let mut vm = brainfuck::VirtualMachine::new(&mut stdin, &mut stdout);
    vm.run(&program)?;
    Ok(())
}
