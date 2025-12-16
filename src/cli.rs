use std::path::{Path, PathBuf};

use clap::{Parser, arg};

/// Command-line interface for the RnR compiler.
#[derive(Debug, Parser)]
#[command(name = "rnr", about = "Run and configure the RnR compiler pipeline")]
pub struct Cli {
    /// Input program to compile; defaults to `./main.rs` when omitted.
    #[arg(
        short = 'i',
        long = "input",
        value_name = "PATH",
        default_value = "main.rs"
    )]
    pub input: PathBuf,

    /// Dump the parsed AST to a file.
    #[arg(short = 'a', long = "ast", value_name = "PATH")]
    pub ast: Option<PathBuf>,

    /// Run the type checker.
    #[arg(short = 't', long = "type-check", alias = "type_check")]
    pub type_check: bool,

    /// Run the virtual machine on generated code.
    #[arg(short = 'v', long = "virtual-machine", alias = "vm")]
    pub virtual_machine: bool,

    /// perform code generation.
    #[arg(short = 'c', long = "code-gen", alias = "code_gen")]
    pub code_gen: bool,

    /// Write generated assembly to a file.
    #[arg(long = "asm", value_name = "PATH")]
    pub asm: Option<PathBuf>,

    /// Execute generated code using the mips VM.
    #[arg(short = 'r', long = "run")]
    pub run: bool,
}

impl Cli {
    /// Parse arguments from the process invocation.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Execute compiler actions based on parsed options.
    pub fn execute(&self) {
        parse_input(&self.input);

        if let Some(ast_path) = &self.ast {
            emit_ast(ast_path);
        }

        if self.type_check {
            run_type_check(&self.input);
        }

        if self.code_gen {
            generate_code(&self.input, self.asm.as_ref());
        }

        if self.virtual_machine {
            run_vm();
        }

        if self.run {
            run_generated();
        }
    }
}

fn parse_input(path: &Path) {
    todo!("Parse the input RnR program at {}", path.display());
}

fn emit_ast(path: &Path) {
    unimplemented!("Higher grade: emit the AST to {}", path.display());
}

fn run_type_check(path: &Path) {
    todo!(
        "Run the type checker on the parsed program from {}",
        path.display()
    );
}

fn generate_code(path: &Path, asm_out: Option<&PathBuf>) {
    if let Some(path) = asm_out {
        unimplemented!(
            "Higher grade: write generated assembly to {}",
            path.display()
        );
    }

    todo!(
        "Generate assembly or bytecode for the program at {}",
        path.display()
    );
}

fn run_vm() {
    unimplemented!("Higher grade: step the virtual machine over generated code");
}

fn run_generated() {
    todo!("Run generated code using the mips VM");
}
