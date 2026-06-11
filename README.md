# RnR Compiler

A small compiler toolchain for the RnR language, including parsing, type checking, a tree-walk VM, and MIPS-like code generation with a VM runner.

## Features

- Parse RnR source files into an AST
- Type check programs
- Execute programs in the tree-walk VM
- Generate MIPS-like instructions and run them with the mips VM
- Dump AST and assembly to files
- A [playground](https://rustpiler.dilred.dev) featuring diagnostics and view of Ast

## Build

```sh
cargo build
```

Release build:

```sh
cargo build --release
```

## CLI Usage

```sh
cargo run -- --help
```

Common flags:

- `-i, --input <PATH>`: input file (default: `examples/ex1.rnr`)
- `-a, --ast <PATH>`: dump AST to file
- `-t, --type_check`: run type checker
- `-v, --virtual_machine`: run tree-walk VM
- `-c, --code_gen`: run code generation
- `--asm <PATH>`: write formatted assembly to file
- `-r`: run generated code in the mips VM

## Run Examples

Parse and type check an example:

```sh
cargo run -- -i examples/ex1.rnr -t
```

Run the tree-walk VM:

```sh
cargo run -- -i examples/ex2.rnr -v
```

Generate assembly and run the mips VM:

```sh
cargo run -- -i examples/ex3_if.rnr --asm out.asm -r
```

Run with release binary:

```sh
./target/release/rnr -i examples/ex4_while_sum.rnr -t -r
```

## Examples Directory

Sample programs live in `examples/`:

- `examples/ex1.rnr`
- `examples/ex2.rnr`
- `examples/ex3_if.rnr`
- `examples/ex4_while_sum.rnr`
- `examples/ex5_block_expr.rnr`

## Notes

- Inline comments start with `//` and are stripped before parsing.
- Code generation currently targets the mips crate instruction set; some features (e.g., strings) are VM-only.
