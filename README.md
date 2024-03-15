# ruBF

ruBF is a Brainfuck interpreter crafted with Rust.

# Build

```
git clone https://github.com/justapig9020/ruBF
cd ruBF
cargo build
```

# Usage

To run a Brainfuck program with ruBF, use the following syntax in your command line:

```
Usage: rubf --program <PROGRAM>

Options:
  -p, --program <PROGRAM>
  -h, --help               Print help
  -V, --version            Print version
```

# Hello world

```
cargo run -- --p ./programs/hello_world.bf
```

# License

ruBF is open-source and available under the MIT License. For more details, see the LICENSE file in the repository.
