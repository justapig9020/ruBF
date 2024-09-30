# ruBF

ruBF is a Brainfuck interpreter crafted with Rust.

## Build

```
git clone https://github.com/justapig9020/ruBF
cd ruBF
cargo build
```

## Usage

To run a Brainfuck program with ruBF, use the following syntax in your command line:

```
Usage: rubf --program <PROGRAM>

Options:
  -p, --program <PROGRAM>
  -h, --help               Print help
  -V, --version            Print version
```

## Hello world

```
cargo run -- --p ./programs/hello_world.bf
```

## Related Projects

This project is part of a series aimed at building a compiler to prove that Brainfuck is Turing complete. You can find the other related projects here:

- [tm-compiler](https://github.com/justapig9020/tm-compiler): Converts Turing machines into a custom C-like IR (bf-c).
- [bf-compiler](https://github.com/justapig9020/bf-compiler): Compiles bf-c programs into Brainfuck.
- [rubf](https://github.com/justapig9020/rubf): A Brainfuck virtual machine.

## License

ruBF is open-source and available under the MIT License. For more details, see the LICENSE file in the repository.
